extern crate bigtable as bt;
#[macro_use]
extern crate error_chain;
extern crate libc;
extern crate goauth;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate protobuf;
extern crate rustc_serialize;

mod fdw_error {
    error_chain! {
        foreign_links {
            FromUtf8(::std::string::FromUtf8Error);
            Utf8(::std::str::Utf8Error);
            Io(::std::io::Error);
            Base64(::rustc_serialize::base64::FromBase64Error);
            Auth(::goauth::error::GOErr);
            Ffi(::std::ffi::NulError);
            Bt(::bt::error::BTErr);
            Json(::serde_json::Error);
        }
    }
}

mod fdw;
#[allow(unused_variables)]
pub mod ffi;
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(improper_ctypes)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
mod pg; // Generated PG bindings
mod structs;

static mut LIMIT: Option<i64> = Some(0);