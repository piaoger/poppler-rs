use super::*;

/// PopplerRectangle:
/// @x1: x coordinate of lower left corner
/// @y1: y coordinate of lower left corner
/// @x2: x coordinate of upper right corner
/// @y2: y coordinate of upper right corner
///
/// A #PopplerRectangle is used to describe
/// locations on a page and bounding boxes
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerRectangle {
    pub x1: gdouble,
    pub y1: gdouble,
    pub x2: gdouble,
    pub y2: gdouble,
}