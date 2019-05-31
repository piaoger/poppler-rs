#![allow(dead_code)]

use super::*;


/// PopplerMediaSaveFunc:
/// @buf: (array length=count) (element-type guint8): buffer containing
///   bytes to be written.
/// @count: number of bytes in @buf.
/// @data: (closure): user data passed to poppler_media_save_to_callback()
/// @error: GError to set on error, or %NULL
///
/// Specifies the type of the function passed to
/// poppler_media_save_to_callback().  It is called once for each block of
/// bytes that is \"written\" by poppler_media_save_to_callback().  If
/// successful it should return %TRUE.  If an error occurs it should set
/// @error and return %FALSE, in which case poppler_media_save_to_callback()
/// will fail with the same error.
///
/// Returns: %TRUE if successful, %FALSE (with @error set) if failed.
///
/// Since: 0.14
pub type PopplerMediaSaveFunc = ::std::option::Option<
    unsafe extern "C" fn(
        buf: *const gchar,
        count: gsize,
        data: gpointer,
        error: *mut *mut GError,
    ) -> gboolean,
>;
