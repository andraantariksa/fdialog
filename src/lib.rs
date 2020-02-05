use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::str::Utf8Error;

extern "C" {
    fn open_dialog() -> *const c_char;
    fn free_string(string: *const c_char);
}

pub struct FileDialog {
    pub multiple: bool,
}

impl FileDialog {
    pub fn ask(&self) -> String {
        let path_output: Result<&str, Utf8Error>;
        let mut output_string = String::new();
        unsafe {
            let path_ptr = open_dialog();
            path_output = CStr::from_ptr(path_ptr).to_str();
            output_string = path_output.unwrap().to_owned();
            free_string(path_ptr);
        }
        return output_string;
    }
}
