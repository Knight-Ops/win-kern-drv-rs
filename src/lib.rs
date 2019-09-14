#![feature(lang_items)]
#![no_std]
#![no_main]

use winapi::shared::ntdef::*;
use winapi::km::wdm::*;

mod lang;

#[no_mangle]
extern "system" fn DriverEntry(pDriverObject: PDRIVER_OBJECT, pRegistryPath: PUNICODE_STRING) -> NTSTATUS {

    unsafe {
        (*pDriverObject).DriverUnload = Some(DriverUnload);
    }

    1
}

#[no_mangle]
extern "system" fn DriverUnload<'r>(pDriverObject: &'r mut DRIVER_OBJECT) {
    
}