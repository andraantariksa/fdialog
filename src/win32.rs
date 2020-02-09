use std::os::raw::c_char;

extern "C" {
    pub fn open_dialog() -> *const c_char;
    // fn free_string(string: *const c_char);
}
