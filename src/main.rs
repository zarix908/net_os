#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(net_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use net_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("run...");

    net_os::init();

    #[cfg(test)]
    test_main();

    println!("didn't crash");
    net_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    net_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    net_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
