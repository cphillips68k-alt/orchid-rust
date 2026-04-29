use crate::{console::kprintln, scheduler, task::Task};

pub fn run() -> ! {
    scheduler::init();
    scheduler::spawn(Task::new("driver", task::driver_task));
    scheduler::spawn(Task::new("service", task::service_task));
    scheduler::spawn(Task::new("shell", task::shell_task));

    kprintln!("Kernel scheduler entering preemptive run loop");

    scheduler::start()
}
