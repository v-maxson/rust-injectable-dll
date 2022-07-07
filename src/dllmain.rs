use std::ffi::CString;
use winapi::{
    shared::minwindef::{BOOL, TRUE, FALSE, HINSTANCE, LPVOID, DWORD}, 
    um::{
        winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH}, 
        libloaderapi::{DisableThreadLibraryCalls, FreeLibraryAndExitThread}, 
        processthreadsapi::CreateThread, winuser::MessageBoxA
    }
};
use crate::_main::main;


#[no_mangle]
unsafe extern "stdcall" fn DllMain(
    hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    _lp_reserved: LPVOID
) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            DisableThreadLibraryCalls(hinst_dll);
            
            if CreateThread(
                std::ptr::null_mut(),
                0,
                Some(main_thread),
                hinst_dll as _,
                0,
                std::ptr::null_mut()
            ).is_null() {
                return FALSE;
            };

        }

        DLL_PROCESS_DETACH => (),
        DLL_THREAD_ATTACH => (),
        DLL_THREAD_DETACH => (),


        _ => ()
    }

    TRUE
}

unsafe extern "system" fn main_thread(base: LPVOID) -> DWORD {
    // Call into main.
    match std::panic::catch_unwind(main) {
        Ok(_) => (),
        Err(e) => {
            let e_msg = CString::new(format!("thread 'main' encountered an error: {:#?}", e));

            let e = match e_msg {
                Ok(msg) => msg.as_ptr(),
                Err(_) => cstr!("thread 'main' encountered an error").as_ptr(),
            };

            MessageBoxA(
                0 as _, 
                cstr!("Error!").as_ptr(),
                e,
                0
            );
        },
    };

    FreeLibraryAndExitThread(base as _, 0);
    unreachable!()
}