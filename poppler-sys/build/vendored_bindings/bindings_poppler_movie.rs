/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _PopplerMovie {
    _unused: [u8; 0],
}
pub const PopplerMoviePlayMode_POPPLER_MOVIE_PLAY_MODE_ONCE: PopplerMoviePlayMode = 0;
pub const PopplerMoviePlayMode_POPPLER_MOVIE_PLAY_MODE_OPEN: PopplerMoviePlayMode = 1;
pub const PopplerMoviePlayMode_POPPLER_MOVIE_PLAY_MODE_REPEAT: PopplerMoviePlayMode = 2;
pub const PopplerMoviePlayMode_POPPLER_MOVIE_PLAY_MODE_PALINDROME: PopplerMoviePlayMode = 3;
pub type PopplerMoviePlayMode = u32;
extern "C" {
    pub fn poppler_movie_get_type() -> GType;
}
extern "C" {
    pub fn poppler_movie_get_filename(poppler_movie: *mut PopplerMovie) -> *const gchar;
}
extern "C" {
    pub fn poppler_movie_need_poster(poppler_movie: *mut PopplerMovie) -> gboolean;
}
extern "C" {
    pub fn poppler_movie_show_controls(poppler_movie: *mut PopplerMovie) -> gboolean;
}
extern "C" {
    pub fn poppler_movie_get_play_mode(poppler_movie: *mut PopplerMovie) -> PopplerMoviePlayMode;
}
