use winapi::{
    ctypes::c_void,
    shared::ntdef::NULL,
    um::{
        handleapi::CloseHandle,
        processthreadsapi::{GetCurrentProcess, OpenProcessToken},
        securitybaseapi::GetTokenInformation,
        winnt::{TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
    },
};

/// Checks if the user is an administrator.
pub fn is_admin() -> bool {
    let mut elevated: bool = false;

    unsafe {
        let process = GetCurrentProcess();

        let mut h_token: *mut c_void = NULL;

        let token = OpenProcessToken(process, TOKEN_QUERY, &mut h_token) != 0;

        if token {
            // This initialization doesn't matter as it will be overwritten by the call to GetTokenInformation.
            let mut elevation: TOKEN_ELEVATION = TOKEN_ELEVATION { TokenIsElevated: 0 };

            let mut ret_size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;

            let token_info = GetTokenInformation(
                h_token,
                TokenElevation,
                &mut elevation as *mut _ as *mut c_void,
                std::mem::size_of_val(&elevation) as u32,
                &mut ret_size,
            ) != 0;

            if token_info {
                elevated = elevation.TokenIsElevated != 0;
            }

            CloseHandle(h_token);
        }
    };

    elevated
}
