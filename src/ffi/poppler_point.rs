use super::*;

/// PopplerPoint:
/// @x: x coordinate
/// @y: y coordinate
///
/// A #PopplerPoint is used to describe a location point on a page
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerPoint {
    pub x: gdouble,
    pub y: gdouble,
}