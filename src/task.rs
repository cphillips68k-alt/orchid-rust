use crate::{console::kprintln, ipc::{IpcHub, Message, MessagePayload}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Runnable,
    Waiting,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskAction {
    Continue,
    Yield,
    Sleep,
    Exit,
}

pub type TaskRoutine = fn(&mut Task, &mut IpcHub) -> TaskAction;

pub struct Task {
    pub id: u32,
    pub name: &'static str,
    pub state: TaskState,
    phase: u8,
    entry: TaskRoutine,
    tick_count: u64,
}

impl Task {
    pub fn new(name: &'static str, entry: TaskRoutine) -> Task {
        Task {
            id: name.as_ptr() as u32,
            name,
            state: TaskState::Runnable,
            phase: 0,
            entry,
            tick_count: 0,
        }
    }

    pub fn step(&mut self, ipc: &mut IpcHub) -> TaskAction {
        if self.state == TaskState::Waiting && ipc.has_message(self.name) {
            self.state = TaskState::Runnable;
        }
        self.tick_count += 1;
        (self.entry)(self, ipc)
    }
}

pub fn driver_task(task: &mut Task, ipc: &mut IpcHub) -> TaskAction {
    match task.phase {
        0 => {
            kprintln!("[driver] probing hardware");
            task.phase = 1;
            TaskAction::Yield
        }
        1 => {
            let message = Message::new("driver", "service", MessagePayload::Text("DEVICE_READY"));
            if ipc.send(message) {
                kprintln!("[driver] notified service");
            }
            task.phase = 2;
            TaskAction::Yield
        }
        2 => {
            if let Some(message) = ipc.receive("driver") {
                kprintln!("[driver] got reply from {}", message.sender);
                task.phase = 3;
                TaskAction::Exit
            } else {
                TaskAction::Sleep
            }
        }
        _ => TaskAction::Exit,
    }
}

pub fn service_task(task: &mut Task, ipc: &mut IpcHub) -> TaskAction {
    match task.phase {
        0 => {
            if let Some(message) = ipc.receive("service") {
                kprintln!("[service] event: {}", match message.payload {
                    MessagePayload::Text(text) => text,
                    MessagePayload::Event(code) => {
                        let _ = code;
                        "event"
                    }
                });
                let reply = Message::new("service", "driver", MessagePayload::Text("ACK"));
                if ipc.send(reply) {
                    kprintln!("[service] acknowledged driver");
                }
                let broadcast = Message::new("service", "shell", MessagePayload::Text("SYSTEM_READY"));
                if ipc.send(broadcast) {
                    kprintln!("[service] notified shell");
                }
                task.phase = 1;
                TaskAction::Continue
            } else {
                TaskAction::Sleep
            }
        }
        1 => {
            TaskAction::Sleep
        }
        _ => TaskAction::Exit,
    }
}

pub fn shell_task(task: &mut Task, ipc: &mut IpcHub) -> TaskAction {
    match task.phase {
        0 => {
            let query = Message::new("shell", "service", MessagePayload::Text("QUERY_STATUS"));
            if ipc.send(query) {
                kprintln!("[shell] asked service for status");
            }
            task.phase = 1;
            TaskAction::Yield
        }
        1 => {
            if let Some(message) = ipc.receive("shell") {
                kprintln!("[shell] service replied with {:?}", message.payload);
                task.phase = 2;
                TaskAction::Exit
            } else {
                TaskAction::Sleep
            }
        }
        _ => TaskAction::Exit,
    }
}
