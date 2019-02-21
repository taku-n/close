// Send Close Values

mod dbg;

use std::mem;
use std::slice;

#[no_mangle]
pub extern "C" fn close(close: *const f64, n: usize) {

    let c = unsafe {slice::from_raw_parts(close, n)};

    dbg::s_f64(&c[..10]);
}
