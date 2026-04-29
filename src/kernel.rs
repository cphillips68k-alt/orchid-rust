use crate::{console::kprintln, ipc::IpcHub, scheduler::Scheduler, task::Task};

pub fn run() -> ! {
    let mut ipc = IpcHub::new();
    let mut scheduler = Scheduler::new(&mut ipc);

    scheduler.spawn(Task::new("driver", task::driver_task));
    scheduler.spawn(Task::new("service", task::service_task));
    scheduler.spawn(Task::new("shell", task::shell_task));

    kprintln!("Kernel scheduler entering main loop");
    scheduler.run()
}
