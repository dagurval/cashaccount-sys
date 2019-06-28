#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use super::*;

    #[test]
    fn accountname() {
        unsafe {
            let name = CString::new("asdf").unwrap();
            assert_eq!(1, cashacc_check_name(name.as_ptr(), 4));
            let name = CString::new("").unwrap();
            assert_eq!(0, cashacc_check_name(name.as_ptr(), 0));
        }
    }
}
