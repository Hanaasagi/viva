use libc;
use std::ffi::CString;
use std::mem::transmute;
use std::os::raw::c_int;
use std::os::raw::c_void;

#[no_mangle]
extern "C" fn write(fd: c_int, buf: *const c_void, count: libc::size_t) -> libc::ssize_t {
    let name = CString::new("write").expect("CString::new failed");
    let ptr = unsafe { libc::dlsym(libc::RTLD_NEXT, name.as_ptr()) };

    let handler: fn(c_int, *const c_void, libc::size_t) -> libc::ssize_t =
        unsafe { transmute(ptr) };

    let rtn: libc::ssize_t;

    if fd == 2 {
        handler(fd, "\x1b[31m" as *const _ as *const c_void, 5);
        rtn = handler(fd, buf, count);
        handler(fd, "\x1b[0m" as *const _ as *const c_void, 4);
    } else {
        rtn = handler(fd, buf, count);
    }
    return rtn;
}
