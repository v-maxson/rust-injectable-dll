// Do not compile for anything other than windows.
#[cfg(not(target_os = "windows"))]
compile_error!("this project can only compile for windows.");

// Only compile when targeting i686-pc-windows-msvc
#[cfg(not(target_arch = "x86"))]
compile_error!("this project can only compile to 32 bit");

#[macro_use]
extern crate cstr;

mod _main;
mod dllmain;