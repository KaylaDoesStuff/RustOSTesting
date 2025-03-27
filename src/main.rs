extern crate user32;
extern crate winapi;

use std::{thread, process};
use std::ffi::CString;
use user32::MessageBoxA;
use winapi::winuser::{MB_OK};
use core::time::Duration;

fn open_url(url: &str) -> bool {
    if let Ok(mut child) = process::Command::new("cmd.exe")
        .arg("/C").arg("start").arg("").arg(&url).spawn() {
        thread::sleep(Duration::new(3, 0));
        if let Ok(status) = child.wait() {
            return status.success();
        }
    }
    false
}

fn main() {
    let lp_text = CString::new("Get looped. >:3").unwrap();
    let lp_caption = CString::new("Have fun.").unwrap();
    let delta_sleep = Duration::from_millis(2500);
    unsafe {
        loop {
            MessageBoxA(
                std::ptr::null_mut(),
                lp_text.as_ptr(),
                lp_caption.as_ptr(),
                MB_OK
            );
            thread::sleep(delta_sleep);
            open_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ");
        }
    }
}
