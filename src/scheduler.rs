use alloc::vec::Vec;
use crate::{console::kprintln, cpu, ipc::IpcHub, task::{Task, TaskAction, TaskState}};

pub struct Scheduler<'ipc> {
    ipc: &'ipc mut IpcHub,
    tasks: Vec<Task>,
}

impl<'ipc> Scheduler<'ipc> {
    pub fn new(ipc: &'ipc mut IpcHub) -> Self {
        Scheduler { ipc, tasks: Vec::new() }
    }

    pub fn spawn(&mut self, task: Task) {
        kprintln!("[scheduler] spawn task {}", task.name);
        self.tasks.push(task);
    }

    pub fn run(&mut self) -> ! {
        loop {
            if self.tasks.is_empty() {
                kprintln!("[scheduler] no runnable tasks, halting");
                cpu::halt_loop();
            }

            let mut made_progress = false;

            for task in &mut self.tasks {
                if task.state == TaskState::Waiting && self.ipc.has_message(task.name) {
                    task.state = TaskState::Runnable;
                }

                if task.state != TaskState::Runnable {
                    continue;
                }

                made_progress = true;
                let action = task.step(self.ipc);

                match action {
                    TaskAction::Continue => {}
                    TaskAction::Yield => {}
                    TaskAction::Sleep => task.state = TaskState::Waiting,
                    TaskAction::Exit => task.state = TaskState::Terminated,
                }
            }

            self.tasks.retain(|task| task.state != TaskState::Terminated);

            if !made_progress {
                kprintln!("[scheduler] all tasks blocked, halting");
                cpu::halt_loop();
            }
        }
    }
}
