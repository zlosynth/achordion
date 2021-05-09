use crate::cstr;

pub fn info(message: &str) {
    let m = cstr::cstr(message);
    unsafe {
        pd_sys::post(m.as_ptr());
    }
}
