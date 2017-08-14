#![allow(unused_mut)]

extern crate backtrace;
extern crate libc;

use std::{panic, env, ptr};
use std::process::Command;
use libc::{c_void, c_int, c_char};
use backtrace::Backtrace;

fn test() {
    panic!("lul");
}

extern "C" {
    fn dlinfo(handle: *mut c_void, request: c_int, info: *mut c_void) -> c_int;
}

const RTLD_DI_LINKMAP: c_int = 2;

#[allow(dead_code)]
struct LinkMap {
    addr: isize,
    name: *mut c_char,
    l_ld: usize,
    l_next: *mut LinkMap,
    l_prev: *mut LinkMap,
}

fn main() {
    println!("Hello, world!");

    panic::set_hook(Box::new(|panic_info| {
        let payload = panic_info.payload().downcast_ref::<String>().map(|x| &**x)
            .or_else(|| panic_info.payload().downcast_ref::<&'static str>().map(|x| *x));
        match payload {
            Some(s) => println!("panic occurred: {:?}", s),
            None => println!("panic occurred with non-string payload"),
        }

        let bt = Backtrace::new();
        if bt.frames().iter().all(|x| x.symbols().is_empty()) {
            let baseptr = unsafe {
                let handle = libc::dlopen(ptr::null(), libc::RTLD_LAZY);
                let mut ptr: *mut LinkMap = ptr::null_mut();
                let ret = dlinfo(handle, RTLD_DI_LINKMAP, (&mut ptr) as *mut _ as *mut c_void);
                assert_eq!(ret, 0);
                (*ptr).addr
            };

            let addrs = bt.frames().iter().map(|x| x.ip().wrapping_offset(-baseptr));

            let exe = env::current_exe().unwrap();
            let dbg = exe.with_extension("dbg");
            if dbg.exists() {
                println!("stack backtrace (symbols from symbol file):");
                Command::new("/usr/bin/addr2line")
                    .arg("-Cipf").arg("-e").arg(exe)
                    .arg("-a").args(addrs.map(|x| format!("{:p}", x)))
                    .status().unwrap();
            } else {
                println!("stack backtrace (no symbols found):");
                for frame in addrs {
                    print!("{:p} ", frame);
                }
                println!();
            }
        } else {
            // have real backtrace (symbols in this binary)
            println!("{:?}", bt);
        }
    }));

    test();
}
