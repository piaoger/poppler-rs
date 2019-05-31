use super::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerAnnotCalloutLine {
    pub multiline: gboolean,
    pub x1: gdouble,
    pub y1: gdouble,
    pub x2: gdouble,
    pub y2: gdouble,
    pub x3: gdouble,
    pub y3: gdouble,
}