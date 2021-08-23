use std::os::raw::{c_char, c_int};
extern "C" {
    pub fn node_start(argc: c_int, argv: *mut *mut c_char) -> c_int;
}
