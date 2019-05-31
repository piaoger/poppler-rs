#![allow(dead_code, non_snake_case)]

use std::os::raw::{c_char, c_int, c_uint};
use super::bitfield;

/// PopplerDest:
/// @type: type of destination
/// @page_num: page number
/// @left: left coordinate
/// @bottom: bottom coordinate
/// @right: right coordinate
/// @top: top coordinate
/// @zoom: scale factor
/// @named_dest: name of the destination (#POPPLER_DEST_NAMED only)
/// @change_left: whether left coordinate should be changed
/// @change_top: whether top coordinate should be changed
/// @change_zoom: whether scale factor should be changed
///
/// Data structure for holding a destination
///
/// Note that @named_dest is the string representation of the named
/// destination. This is the right form to pass to poppler functions,
/// e.g. poppler_document_find_dest(), but to get the destination as
/// it appears in the PDF itself, you need to convert it to a bytestring
/// with poppler_named_dest_to_bytestring() first.
/// Also note that @named_dest does not have a defined encoding and
/// is not in a form suitable to be displayed to the user."
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerDest {
    pub type_: PopplerDestType,
    pub page_num: c_int,
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
    pub top: f64,
    pub zoom: f64,
    pub named_dest: *mut c_char,
    pub _bitfield_1: bitfield::__BindgenBitfieldUnit<[u8; 1usize], u8>,
    pub __bindgen_padding_0: [u8; 7usize],
}

impl PopplerDest {
    #[inline]
    pub fn change_left(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_change_left(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn change_top(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_change_top(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn change_zoom(&self) -> c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_change_zoom(&mut self, val: c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        change_left: c_uint,
        change_top: c_uint,
        change_zoom: c_uint,
    ) -> bitfield::__BindgenBitfieldUnit<[u8; 1usize], u8> {
        let mut __bindgen_bitfield_unit: bitfield::__BindgenBitfieldUnit<[u8; 1usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let change_left: u32 = unsafe { ::std::mem::transmute(change_left) };
            change_left as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let change_top: u32 = unsafe { ::std::mem::transmute(change_top) };
            change_top as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let change_zoom: u32 = unsafe { ::std::mem::transmute(change_zoom) };
            change_zoom as u64
        });
        __bindgen_bitfield_unit
    }
}

/// PopplerDestType:
/// @POPPLER_DEST_UNKNOWN: unknown destination
/// @POPPLER_DEST_XYZ: go to page with coordinates (left, top)
/// positioned at the upper-left corner of the window and the contents of
/// the page magnified by the factor zoom
/// @POPPLER_DEST_FIT: go to page with its contents magnified just
/// enough to fit the entire page within the window both horizontally and
/// vertically
/// @POPPLER_DEST_FITH: go to page with the vertical coordinate top
/// positioned at the top edge of the window and the contents of the page
/// magnified just enough to fit the entire width of the page within the window
/// @POPPLER_DEST_FITV: go to page with the horizontal coordinate
/// left positioned at the left edge of the window and the contents of the
/// page magnified just enough to fit the entire height of the page within the window
/// @POPPLER_DEST_FITR: go to page with its contents magnified just
/// enough to fit the rectangle specified by the coordinates left, bottom,
/// right, and top entirely within the window both horizontally and vertically
/// @POPPLER_DEST_FITB: go to page with its contents magnified just enough to fit
/// its bounding box entirely within the window both horizontally and vertically
/// @POPPLER_DEST_FITBH: go to page with the vertical
/// coordinate top positioned at the top edge of the window and the
/// contents of the page magnified just enough to fit the entire width of its
/// bounding box within the window
/// @POPPLER_DEST_FITBV: go to page with the horizontal
/// coordinate left positioned at the left edge of the window and the
/// contents of the page magnified just enough to fit the entire height of its
/// bounding box within the window
/// @POPPLER_DEST_NAMED: got to page specified by a name. See poppler_document_find_dest()
///
/// Destination types
pub type PopplerDestType = u32;

#[test]
fn bindgen_test_layout_PopplerDest() {
    assert_eq!(
        ::std::mem::size_of::<PopplerDest>(),
        64usize,
        concat!("Size of: ", stringify!(PopplerDest))
    );
    assert_eq!(
        ::std::mem::align_of::<PopplerDest>(),
        8usize,
        concat!("Alignment of ", stringify!(PopplerDest))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerDest>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerDest),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerDest>())).page_num as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerDest),
            "::",
            stringify!(page_num)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerDest>())).left as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerDest),
            "::",
            stringify!(left)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerDest>())).bottom as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerDest),
            "::",
            stringify!(bottom)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerDest>())).right as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerDest),
            "::",
            stringify!(right)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerDest>())).top as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerDest),
            "::",
            stringify!(top)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerDest>())).zoom as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerDest),
            "::",
            stringify!(zoom)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerDest>())).named_dest as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerDest),
            "::",
            stringify!(named_dest)
        )
    );
}