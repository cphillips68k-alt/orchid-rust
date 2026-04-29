use crate::{console::kprintln, cpu, ipc::{IpcHub, Message}, task::{Task, TaskState}};
use core::arch::asm;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

pub struct Scheduler {
    ipc: IpcHub,
    tasks: Vec<Task>,
    current: usize,
    started: bool,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            ipc: IpcHub::new(),
            tasks: Vec::new(),
            current: 0,
            started: false,
        }
    }

    pub fn spawn(&mut self, task: Task) {
        kprintln!("[scheduler] spawn task {}", task.name);
        self.tasks.push(task);
    }

    fn cleanup_terminated(&mut self) {
        self.tasks.retain(|task| task.state != TaskState::Terminated);
        if self.current >= self.tasks.len() {
            self.current = 0;
        }
    }

    fn first_runnable(&self) -> Option<usize> {
        self.tasks.iter().position(|task| task.is_runnable())
    }

    fn next_runnable_after(&self, current: usize) -> Option<usize> {
        let len = self.tasks.len();
        if len == 0 {
            return None;
        }

        for offset in 1..=len {
            let index = (current + offset) % len;
            if self.tasks[index].is_runnable() {
                return Some(index);
            }
        }
        None
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

pub fn start() -> ! {
    let mut scheduler_lock = SCHEDULER.lock();
    let scheduler = scheduler_lock.as_mut().expect("Scheduler must be initialized");
    let first = scheduler.first_runnable().unwrap_or_else(|| {
        kprintln!("[scheduler] no runnable tasks at start");
        cpu::halt_loop()
    });
    scheduler.current = first;
    scheduler.started = true;
    let rsp = scheduler.tasks[first].stack_pointer;
    drop(scheduler_lock);

    task_resume(rsp)
}

#[no_mangle]
pub extern "C" fn scheduler_tick(current_rsp: u64) -> u64 {
    let mut scheduler_lock = SCHEDULER.lock();
    let scheduler = scheduler_lock.as_mut().expect("Scheduler must be initialized");

    scheduler.cleanup_terminated();

    if scheduler.tasks.is_empty() {
        cpu::halt_loop();
    }

    if scheduler.started {
        scheduler.tasks[scheduler.current].stack_pointer = current_rsp;
    }

    let next = scheduler.next_runnable_after(scheduler.current)
        .unwrap_or(scheduler.current);
    scheduler.current = next;
    scheduler.started = true;
    scheduler.tasks[next].stack_pointer
}

pub fn ipc_send(message: Message) -> bool {
    interrupts::without_interrupts(|| {
        if let Some(scheduler) = SCHEDULER.lock().as_mut() {
            scheduler.ipc.send(message)
        } else {
            false
        }
    })
}

pub fn ipc_receive(recipient: &'static str) -> Option<Message> {
    interrupts::without_interrupts(|| {
        if let Some(scheduler) = SCHEDULER.lock().as_mut() {
            scheduler.ipc.receive(recipient)
        } else {
            None
        }
    })
}

pub fn exit_current_task() -> ! {
    let mut scheduler_lock = SCHEDULER.lock();
    let scheduler = scheduler_lock.as_mut().expect("Scheduler must be initialized");
    scheduler.tasks[scheduler.current].state = TaskState::Terminated;
    scheduler.cleanup_terminated();

    if scheduler.tasks.is_empty() {
        cpu::halt_loop();
    }

    let next = scheduler.next_runnable_after(scheduler.current)
        .unwrap_or(0);
    scheduler.current = next;
    let rsp = scheduler.tasks[next].stack_pointer;
    drop(scheduler_lock);

    task_resume(rsp)
}

#[naked]
#[no_mangle]
pub extern "C" fn task_resume(next_rsp: u64) -> ! {
    unsafe {
        asm!(
            "mov rsp, rdi",
            "pop r15",
            "pop r14",
            "pop r13",
            "pop r12",
            "pop r11",
            "pop r10",
            "pop r9",
            "pop r8",
            "pop rbp",
            "pop rdi",
            "pop rsi",
            "pop rdx",
            "pop rcx",
            "pop rbx",
            "pop rax",
            "iretq",
            options(noreturn)
        );
    }
}
