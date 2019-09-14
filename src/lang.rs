
#[lang = "eh_personality"]
#[no_mangle]
extern fn rust_eh_personality() {}

#[lang = "panic_impl"]
extern fn panic_impl(info: &core::panic::PanicInfo) -> ! {

    // Force a null pointer read to crash.
    let _: i32 = unsafe { *(core::ptr::null()) };

    // If that doesn't work, loop forever.
    loop{}
}

#[no_mangle]
pub static _fltused: u32 = 0;