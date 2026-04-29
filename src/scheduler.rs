use alloc::vec::Vec;
use crate::{console::kprintln, cpu, ipc::IpcHub, task::{Task, TaskAction, TaskState}};
use lazy_static::lazy_static;
use spin::Mutex;

pub struct Scheduler {
    ipc: IpcHub,
    tasks: Vec<Task>,
    current: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            ipc: IpcHub::new(),
            tasks: Vec::new(),
            current: 0,
        }
    }

    pub fn spawn(&mut self, task: Task) {
        kprintln!("[scheduler] spawn task {}", task.name);
        self.tasks.push(task);
    }

    fn wake_waiting(&mut self) {
        for task in &mut self.tasks {
            if task.state == TaskState::Waiting && self.ipc.has_message(task.name) {
                task.state = TaskState::Runnable;
            }
        }
    }

    fn find_next_runnable(&mut self) -> Option<usize> {
        let len = self.tasks.len();
        if len == 0 {
            return None;
        }

        for offset in 0..len {
            let index = (self.current + offset) % len;
            if self.tasks[index].state == TaskState::Runnable {
                return Some(index);
            }
        }
        None
    }

    fn remove_terminated(&mut self) {
        self.tasks.retain(|task| task.state != TaskState::Terminated);
        if self.current >= self.tasks.len() {
            self.current = 0;
        }
    }

    pub fn tick(&mut self) {
        self.wake_waiting();

        if self.tasks.is_empty() {
            return;
        }

        let next_index = match self.find_next_runnable() {
            Some(index) => index,
            None => {
                kprintln!("[scheduler] all tasks blocked, waiting for interrupt");
                return;
            }
        };

        let action = {
            let task = &mut self.tasks[next_index];
            task.step(&mut self.ipc)
        };

        match action {
            TaskAction::Continue => {
                self.current = (next_index + 1) % self.tasks.len();
            }
            TaskAction::Yield => {
                self.current = (next_index + 1) % self.tasks.len();
            }
            TaskAction::Sleep => {
                self.tasks[next_index].state = TaskState::Waiting;
                self.current = (next_index + 1) % self.tasks.len();
            }
            TaskAction::Exit => {
                self.tasks[next_index].state = TaskState::Terminated;
                self.remove_terminated();
            }
        }
    }
}

lazy_static! {
    static ref SCHEDULER: Mutex<Option<Scheduler>> = Mutex::new(None);
}

pub fn init() {
    *SCHEDULER.lock() = Some(Scheduler::new());
}

pub fn spawn(task: Task) {
    if let Some(scheduler) = SCHEDULER.lock().as_mut() {
        scheduler.spawn(task);
    }
}

pub fn tick() {
    if let Some(scheduler) = SCHEDULER.lock().as_mut() {
        scheduler.tick();
    }
}
