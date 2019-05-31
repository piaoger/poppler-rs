#![allow(dead_code, non_camel_case_types)]

use super::*;

type guint16 = c_ushort;

/// PopplerColor:
/// @red: the red component of color
/// @green: the green component of color
/// @blue: the blue component of color
///
/// A #PopplerColor describes a RGB color. Color components
/// are values between 0 and 65535
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerColor {
    pub red: guint16,
    pub green: guint16,
    pub blue: guint16,
}