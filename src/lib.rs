#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn get_version() {
        use crate::GetWDSPVersion;
        let v = unsafe { 
            GetWDSPVersion()
        };
        assert_eq!(118, v);
    }
    #[test]
    fn check_wisdom() {
        // major headache, argument declared as char *, but should be const char *
        use crate::WDSPwisdom;
        use std::ffi::CString;

        let directory = CString::new(".").expect("CString::new failed");
        let ptr = directory.as_ptr();
        unsafe {
            WDSPwisdom(ptr);
        }        
    }
}
