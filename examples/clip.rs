use std::{thread, time::Duration};

use quork::clip::open_clipboard;

fn main() {
    open_clipboard().unwrap();

    thread::sleep(Duration::from_secs(1000));
}
