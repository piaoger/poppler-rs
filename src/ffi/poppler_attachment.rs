#![allow(dead_code)]

use super::*;
use gobject_sys::{GObjectClass, GObject};
use glib_sys::{GTime, GString};


/// PopplerAttachmentSaveFunc:
/// @buf: (array length=count) (element-type guint8): buffer containing
///   bytes to be written.
/// @count: number of bytes in @buf.
/// @data: (closure): user data passed to poppler_attachment_save_to_callback()
/// @error: GError to set on error, or %NULL
///
/// Specifies the type of the function passed to
/// poppler_attachment_save_to_callback().  It is called once for each block of
/// bytes that is \"written\" by poppler_attachment_save_to_callback().  If
/// successful it should return %TRUE.  If an error occurs it should set
/// @error and return %FALSE, in which case poppler_attachment_save_to_callback()
/// will fail with the same error.
///
/// Returns: %TRUE if successful, %FALSE (with @error set) if failed.
pub type PopplerAttachmentSaveFunc = ::std::option::Option<
    unsafe extern "C" fn(
        buf: *const gchar,
        count: gsize,
        data: gpointer,
        error: *mut *mut GError,
    ) -> gboolean,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerAttachmentClass {
    pub parent_class: GObjectClass,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerAttachment {
    pub parent: GObject,
    pub name: *mut gchar,
    pub description: *mut gchar,
    pub size: gsize,
    pub mtime: GTime,
    pub ctime: GTime,
    pub checksum: *mut GString,
}

