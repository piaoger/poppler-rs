#![allow(non_snake_case)]

use std::os::raw::{c_char, c_int};

pub mod consts {
    pub const PopplerActionType_POPPLER_ACTION_UNKNOWN: PopplerActionType = 0;
    pub const PopplerActionType_POPPLER_ACTION_NONE: PopplerActionType = 1;
    pub const PopplerActionType_POPPLER_ACTION_GOTO_DEST: PopplerActionType = 2;
    pub const PopplerActionType_POPPLER_ACTION_GOTO_REMOTE: PopplerActionType = 3;
    pub const PopplerActionType_POPPLER_ACTION_LAUNCH: PopplerActionType = 4;
    pub const PopplerActionType_POPPLER_ACTION_URI: PopplerActionType = 5;
    pub const PopplerActionType_POPPLER_ACTION_NAMED: PopplerActionType = 6;
    pub const PopplerActionType_POPPLER_ACTION_MOVIE: PopplerActionType = 7;
    pub const PopplerActionType_POPPLER_ACTION_RENDITION: PopplerActionType = 8;
    pub const PopplerActionType_POPPLER_ACTION_OCG_STATE: PopplerActionType = 9;
    pub const PopplerActionType_POPPLER_ACTION_JAVASCRIPT: PopplerActionType = 10;
    /// PopplerActionType:
    /// @POPPLER_ACTION_UNKNOWN: unknown action
    /// @POPPLER_ACTION_NONE: no action specified
    /// @POPPLER_ACTION_GOTO_DEST: go to destination
    /// @POPPLER_ACTION_GOTO_REMOTE: go to destination in another document
    /// @POPPLER_ACTION_LAUNCH: launch app (or open document
    /// @POPPLER_ACTION_URI: URI
    /// @POPPLER_ACTION_NAMED: predefined action
    /// @POPPLER_ACTION_MOVIE: play movies. Since 0.14
    /// @POPPLER_ACTION_RENDITION: play multimedia content. Since 0.14
    /// @POPPLER_ACTION_OCG_STATE: state of layer. Since 0.14
    /// @POPPLER_ACTION_JAVASCRIPT: Javascript. Since 0.18
    ///
    /// Action types
    pub type PopplerActionType = u32;


    pub const PopplerActionMovieOperation_POPPLER_ACTION_MOVIE_PLAY: PopplerActionMovieOperation = 0;
    pub const PopplerActionMovieOperation_POPPLER_ACTION_MOVIE_PAUSE: PopplerActionMovieOperation = 1;
    pub const PopplerActionMovieOperation_POPPLER_ACTION_MOVIE_RESUME: PopplerActionMovieOperation = 2;
    pub const PopplerActionMovieOperation_POPPLER_ACTION_MOVIE_STOP: PopplerActionMovieOperation = 3;
    /// PopplerActionMovieOperation:
    /// @POPPLER_ACTION_MOVIE_PLAY: play movie
    /// @POPPLER_ACTION_MOVIE_PAUSE: pause playing movie
    /// @POPPLER_ACTION_MOVIE_RESUME: resume paused movie
    /// @POPPLER_ACTION_MOVIE_STOP: stop playing movie
    ///
    /// Movie operations
    ///
    /// Since: 0.14
    pub type PopplerActionMovieOperation = u32;
}



#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerActionJavascript {
    pub type_: consts::PopplerActionType,
    pub title: *mut c_char,
    pub script: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerActionOCGState {
    pub type_: consts::PopplerActionType,
    pub title: *mut c_char,
    pub state_list: *mut glib_sys::GList,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerActionRendition {
    pub type_: consts::PopplerActionType,
    pub title: *mut c_char,
    pub op: c_int,
    pub media: *mut super::PopplerMedia,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerActionMovie {
    pub type_: consts::PopplerActionType,
    pub title: *mut c_char,
    pub operation: consts::PopplerActionMovieOperation,
    pub movie: *mut super::PopplerMovie,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerActionNamed {
    pub type_: consts::PopplerActionType,
    pub title: *mut c_char,
    pub named_dest: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerActionUri {
    pub type_: consts::PopplerActionType,
    pub title: *mut c_char,
    pub uri: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerActionLaunch {
    pub type_: consts::PopplerActionType,
    pub title: *mut c_char,
    pub file_name: *mut c_char,
    pub params: *mut c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerActionGotoRemote {
    pub type_: consts::PopplerActionType,
    pub title: *mut c_char,
    pub file_name: *mut c_char,
    pub dest: *mut super::poppler_dest::PopplerDest,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerActionGotoDest {
    pub type_: consts::PopplerActionType,
    pub title: *mut c_char,
    pub dest: *mut super::poppler_dest::PopplerDest,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerActionRendition {
    pub type_: consts::PopplerActionType,
    pub title: *mut c_char,
    pub op: c_int,
    pub media: *mut super::PopplerMedia,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerActionAny {
    pub type_: consts::PopplerActionType,
    pub title: *mut c_char,
}

// /// PopplerActionLayer:
// /// @action: a #PopplerActionLayerAction
// /// @layers: list of #PopplerLayer<!-- -->s
// ///
// /// Action to perform over a list of layers
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct PopplerActionLayer {
//     pub action: PopplerActionLayerAction,
//     pub layers: *mut glib_sys::GList,
// }

/// PopplerAction:
///
/// A data structure for holding actions
#[repr(C)]
#[derive(Copy, Clone)]
pub union PopplerAction {
    pub type_: consts::PopplerActionType,
    pub any: PopplerActionAny,
    pub goto_dest: PopplerActionGotoDest,
    pub goto_remote: PopplerActionGotoRemote,
    pub launch: PopplerActionLaunch,
    pub uri: PopplerActionUri,
    pub named: PopplerActionNamed,
    pub movie: PopplerActionMovie,
    pub rendition: PopplerActionRendition,
    pub ocg_state: PopplerActionOCGState,
    pub javascript: PopplerActionJavascript,
    _bindgen_union_align: [u64; 4usize],
}

extern "C" {
    pub fn poppler_action_get_type() -> glib_sys::GType;
    pub fn poppler_action_free(action: *mut PopplerAction);
    pub fn poppler_action_copy(action: *mut PopplerAction) -> *mut PopplerAction;
    pub fn poppler_dest_get_type() -> glib_sys::GType;
}

#[test]
fn bindgen_test_layout_PopplerAction() {
    assert_eq!(
        ::std::mem::size_of::<PopplerAction>(),
        32usize,
        concat!("Size of: ", stringify!(PopplerAction))
    );
    assert_eq!(
        ::std::mem::align_of::<PopplerAction>(),
        8usize,
        concat!("Alignment of ", stringify!(PopplerAction))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerAction>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerAction),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerAction>())).any as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerAction),
            "::",
            stringify!(any)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerAction>())).goto_dest as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerAction),
            "::",
            stringify!(goto_dest)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerAction>())).goto_remote as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerAction),
            "::",
            stringify!(goto_remote)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerAction>())).launch as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerAction),
            "::",
            stringify!(launch)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerAction>())).uri as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerAction),
            "::",
            stringify!(uri)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerAction>())).named as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerAction),
            "::",
            stringify!(named)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerAction>())).movie as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerAction),
            "::",
            stringify!(movie)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerAction>())).rendition as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerAction),
            "::",
            stringify!(rendition)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerAction>())).ocg_state as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerAction),
            "::",
            stringify!(ocg_state)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<PopplerAction>())).javascript as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(PopplerAction),
            "::",
            stringify!(javascript)
        )
    );
}
