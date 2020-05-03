use libc;

use runas::Command;

pub fn is_admin() -> bool {
    #[cfg(windows)]
    {
        use std::mem;
        use std::ptr::null_mut;
        use winapi::um::handleapi::CloseHandle;
        use winapi::um::processthreadsapi::GetCurrentProcess;
        use winapi::um::processthreadsapi::OpenProcessToken;
        use winapi::um::securitybaseapi::GetTokenInformation;
        use winapi::um::winnt::TokenElevation;
        use winapi::um::winnt::HANDLE;
        use winapi::um::winnt::TOKEN_ELEVATION;
        use winapi::ctypes::c_void;
        use winapi::um::winnt::TOKEN_QUERY;

        let mut handle: HANDLE = null_mut();
        unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) };
    
        let elevation = unsafe { libc::malloc(mem::size_of::<TOKEN_ELEVATION>()) as *mut c_void };
        let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
        let mut ret_size = size;
        unsafe {
            GetTokenInformation(
                handle,
                TokenElevation,
                elevation,
                size as u32,
                &mut ret_size,
            )
        };
        let elevation_struct: TOKEN_ELEVATION = unsafe{ *(elevation as *mut TOKEN_ELEVATION) };
    
        if !handle.is_null() {
            unsafe {
                CloseHandle(handle);
            }
        }
    
        elevation_struct.TokenIsElevated == 1
    }

    #[cfg(unix)]
    {
        unsafe { libc::getuid() == 0 }
    }
}

pub fn host_run_as_admin(active: bool) -> bool {
    #[cfg(windows)]
    {
        if active {
            Command::new(std::env::current_exe().unwrap().to_str().unwrap())
                .args(&["switch", "akatsuki"])
                .show(false)
                .force_prompt(true)
                .status()
                .unwrap()
                .success()
        } else {
            Command::new(std::env::current_exe().unwrap().to_str().unwrap())
                .args(&["switch", "bancho"])
                .show(false)
                .force_prompt(true)
                .status()
                .unwrap()
                .success()
        }
    }

    #[cfg(unix)] // TODO: Figure this out
    {
    }
}

pub fn cert_run_as_admin() -> bool {
    #[cfg(windows)]
    {
        Command::new(std::env::current_exe().unwrap().to_str().unwrap())
            .args(&["certificate", "install"])
            .show(false)
            .force_prompt(true)
            .status()
            .unwrap()
            .success()
    }

    #[cfg(unix)] // TODO: Figure this out
    {
    }
}
