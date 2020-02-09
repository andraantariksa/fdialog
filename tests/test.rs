extern crate fdialog;

use fdialog::*;
use std::ffi::CStr;

#[test]
fn dialog_test() {
    // let file_dialog = FileDialog{
    //     multiple: true,
    // };
    // let out = file_dialog.ask();
    // println!("{:?}", out);
    let out;
    unsafe {
        out = win32::open_dialog();
        println!("In Rust: {:?}", CStr::from_ptr(out));
    }
}
