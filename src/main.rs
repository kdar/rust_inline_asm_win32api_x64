#![feature(asm)]

extern crate winapi;
extern crate kernel32;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::LPCSTR;

fn to_wstring(s: &str) -> Vec<u16> {
    let v: Vec<u16> = OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect();
    v
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn main() {
    let title = "Horray".as_ptr();
    let message = "YEZZZ".as_ptr();
    let symbol = to_wstring("user32.dll");
    let handle = unsafe { kernel32::LoadLibraryW(symbol.as_ptr()) };
    let symbol = "MessageBoxA";
    let messageboxa = unsafe { kernel32::GetProcAddress(handle, symbol.as_ptr() as LPCSTR) };

    unsafe {
        asm!("
      mov rcx, 0
      mov rdx, $1
      mov r8, $0
      mov r9d, 0
      call $2
      "
      :
      : "rm"(title), "rm"(message), "rm"(messageboxa)
      :
      : "intel");
    };
}
