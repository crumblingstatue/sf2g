#![allow(non_camel_case_types)]

macro_rules! decl_opaque {
    ($($(#[$attr:meta])* $v:vis $name:ident;)+) => {
        $(
            $(#[$attr])*
            #[repr(C)]
            #[allow(missing_copy_implementations)]
            $v struct $name {
                _opaque: [u8; 0],
            }
            impl ::std::fmt::Debug for $name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, concat!(stringify!($name), "(<opaque> @ {:p})"), self)
                }
            }
        )+
    };
}

pub(crate) mod graphics;
pub(crate) mod system;
pub(crate) mod window;

use {
    crate::system::{Vector2, Vector3},
    std::{
        ffi::c_void,
        os::raw::{c_char, c_int, c_uint},
    },
};

pub type sfVector3f = Vector3<f32>;
pub type sfVector2i = Vector2<c_int>;
pub type sfVector2u = Vector2<c_uint>;
pub type sfVector2f = Vector2<f32>;
