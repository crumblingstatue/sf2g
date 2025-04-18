pub use crate::ffi::*;
use crate::{
    cpp::{CppString as sfStdString, CppStringVector as sfStdStringVector},
    system::{Clock as sfClock, SfString as sfString},
};

pub type sfTime = i64;

include!("system_bindgen.rs");
