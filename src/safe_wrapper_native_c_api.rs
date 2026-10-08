use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use crate::mock_c_api::get_status_message;

pub mod mock_c_api {
    use super::*;

    #[repr(C)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
    pub struct CServerStats {
        pub active_connections: u32,
        pub error_code: c_int,
    }

    pub extern "C" fn get_status_message(code: c_int) -> *const c_char {
        match code {
            0 => "OK\0".as_ptr() as *const c_char,
            1 => "CONNECTION_TIMEOUT\0".as_ptr() as *const c_char,
            _ => "UNKNOWN_ERROR\0".as_ptr() as *const c_char,
        }
    }

    pub unsafe extern "C" fn fetch_stats_for_user(
        username: *const c_char,
        out_stats: *mut CServerStats,
    ) -> c_int {
        if username.is_null() || out_stats.is_null() {
            return -1;
        }
        let name = unsafe { CStr::from_ptr(username).to_str().unwrap_or("") };
        if name == "admin" {
            unsafe {
                *out_stats = CServerStats { active_connections: 42, error_code: 0 };
            }
            0
        } else {
            unsafe {
                *out_stats = CServerStats { active_connections: 0, error_code: 1 };
            }
            1
        }
    }
}

pub fn format_c_status(code: i32) -> String {
    let status_ptr = get_status_message(code);
    let status_cstr = unsafe { CStr::from_ptr(status_ptr) };
    status_cstr.to_str().unwrap_or("").to_string()
}

pub fn safe_fetch_user_stats(username: &str) -> Result<mock_c_api::CServerStats, String> {
    let username_cstr = CString::new(username);
    if username_cstr.is_err() {
        return Err("Invalid username string".to_string());
    }
    let mut c_server_stats = mock_c_api::CServerStats::default();
    let error_code = unsafe {
        mock_c_api::fetch_stats_for_user(username_cstr.unwrap().as_ptr(), &mut c_server_stats)
    };
    if error_code == 0 {
        Ok(c_server_stats)
    } else {
        Err(format_c_status(c_server_stats.error_code))
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_status_formatter() {
        assert_eq!(format_c_status(0), "OK");
        assert_eq!(format_c_status(1), "CONNECTION_TIMEOUT");
        assert_eq!(format_c_status(99), "UNKNOWN_ERROR");
    }

    #[test]
    fn test_safe_c_interop_wrapper() {
        // Test 1: Successful query for admin user
        let admin_stats = safe_fetch_user_stats("admin").expect("Admin query failed");
        assert_eq!(admin_stats.active_connections, 42);
        assert_eq!(admin_stats.error_code, 0);

        // Test 2: Error query for non-admin user
        let err_result = safe_fetch_user_stats("guest");
        assert!(err_result.is_err());
        assert_eq!(err_result.unwrap_err(), "CONNECTION_TIMEOUT");

        // Test 3: String containing internal null byte must return Err
        let invalid_input = safe_fetch_user_stats("bad\0user");
        assert!(invalid_input.is_err());
        assert_eq!(invalid_input.unwrap_err(), "Invalid username string");
    }
}