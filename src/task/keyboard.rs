use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

use crate::println;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if queue.push(scancode).is_err() {
            println!("WARNING: scancode queue full");
        }
    } else {
        println!("WARNING: scancode queue uninitialized")
    }
}
