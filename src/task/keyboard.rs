use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

use crate::println;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

pub struct ScancodeStream {
    _private: (), // prevents construction of this struct from outside of the module
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("The `new` method should be called once");

        ScancodeStream { _private: () }
    }
}

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if queue.push(scancode).is_err() {
            println!("WARNING: scancode queue full");
        }
    } else {
        println!("WARNING: scancode queue uninitialized")
    }
}
