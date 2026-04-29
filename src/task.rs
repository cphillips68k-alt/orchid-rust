use crate::{console::kprintln, scheduler};
use x86_64::instructions::hlt;

const STACK_SIZE: usize = 4096 * 8;
const MAX_TASKS: usize = 8;

static mut TASK_STACKS: [[u8; STACK_SIZE]; MAX_TASKS] = [[0; STACK_SIZE]; MAX_TASKS];
static mut NEXT_STACK_INDEX: usize = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Runnable,
    Waiting,
    Terminated,
}

pub type TaskRoutine = fn() -> !;

pub struct Task {
    pub id: u32,
    pub name: &'static str,
    pub state: TaskState,
    pub stack_pointer: u64,
    entry: TaskRoutine,
}

impl Task {
    pub fn new(name: &'static str, entry: TaskRoutine) -> Task {
        let stack = unsafe {
            let index = NEXT_STACK_INDEX;
            NEXT_STACK_INDEX = NEXT_STACK_INDEX.saturating_add(1);
            &mut TASK_STACKS[index]
        };

        Task {
            id: name.as_ptr() as u32,
            name,
            state: TaskState::Runnable,
            stack_pointer: Task::initialize_stack(stack, entry),
            entry,
        }
    }

    fn initialize_stack(stack: &'static mut [u8; STACK_SIZE], entry: TaskRoutine) -> u64 {
        let stack_start = stack.as_ptr() as usize + STACK_SIZE;
        let mut stack_top = stack_start & !0xF;

        macro_rules! push_u64 {
            ($value:expr) => {{
                stack_top -= core::mem::size_of::<u64>();
                unsafe { *(stack_top as *mut u64) = $value };
            }};
        }

        push_u64!(0); // rax
        push_u64!(0); // rbx
        push_u64!(0); // rcx
        push_u64!(0); // rdx
        push_u64!(0); // rsi
        push_u64!(0); // rdi
        push_u64!(0); // rbp
        push_u64!(0); // r8
        push_u64!(0); // r9
        push_u64!(0); // r10
        push_u64!(0); // r11
        push_u64!(0); // r12
        push_u64!(0); // r13
        push_u64!(0); // r14
        push_u64!(0); // r15

        push_u64!(entry as u64);
        push_u64!(0x08u64);
        push_u64!(0x202u64);

        stack_top as u64
    }

    pub fn is_runnable(&self) -> bool {
        self.state == TaskState::Runnable
    }
}

pub fn driver_task() -> ! {
    kprintln!("[driver] probing hardware");
    let message = crate::ipc::Message::new("driver", "service", crate::ipc::MessagePayload::Text("DEVICE_READY"));
    if scheduler::ipc_send(message) {
        kprintln!("[driver] notified service");
    }

    loop {
        if let Some(message) = scheduler::ipc_receive("driver") {
            kprintln!("[driver] got reply from {}", message.sender);
            break;
        }
        hlt();
    }

    kprintln!("[driver] driver task exiting");
    scheduler::exit_current_task();
}

pub fn service_task() -> ! {
    loop {
        if let Some(message) = scheduler::ipc_receive("service") {
            let payload_text = match message.payload {
                crate::ipc::MessagePayload::Text(text) => text,
                crate::ipc::MessagePayload::Event(code) => {
                    let _ = code;
                    "event"
                }
            };
            kprintln!("[service] event: {}", payload_text);

            let reply = crate::ipc::Message::new("service", "driver", crate::ipc::MessagePayload::Text("ACK"));
            if scheduler::ipc_send(reply) {
                kprintln!("[service] acknowledged driver");
            }

            let broadcast = crate::ipc::Message::new("service", "shell", crate::ipc::MessagePayload::Text("SYSTEM_READY"));
            if scheduler::ipc_send(broadcast) {
                kprintln!("[service] notified shell");
            }

            kprintln!("[service] service task exiting");
            scheduler::exit_current_task();
        }
        hlt();
    }
}

pub fn shell_task() -> ! {
    kprintln!("[shell] asking service for status");
    let query = crate::ipc::Message::new("shell", "service", crate::ipc::MessagePayload::Text("QUERY_STATUS"));
    if scheduler::ipc_send(query) {
        kprintln!("[shell] asked service for status");
    }

    loop {
        if let Some(message) = scheduler::ipc_receive("shell") {
            kprintln!("[shell] service replied with {:?}", message.payload);
            break;
        }
        hlt();
    }

    kprintln!("[shell] shell task exiting");
    scheduler::exit_current_task();
}
