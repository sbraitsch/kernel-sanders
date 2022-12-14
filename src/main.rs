#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod  vga_buffer;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World! Here are some numbers: {} {}", 13, 37);

    // compiles only with cargo test
    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("Trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    // using port io to tell qemu to close
    // this behaviour is defined through the bootimage dependency (see cargo.toml)
    // any write to 0xf4 causes qemu to exit with (value << 1) | 1 (success = 33, failure = 35)
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
