#![allow(dead_code, non_upper_case_globals, non_camel_case_types)]

pub mod poppler_dest;
pub mod poppler_action;
pub mod poppler_attachment;
pub mod poppler_annot;
pub mod poppler_media;
pub mod poppler_rectangle;
pub mod poppler_point;
pub mod poppler_color;
pub mod poppler_page;
pub mod bitfield;
pub mod consts;

use glib_sys;
use std::os::raw::{c_char, c_int, c_uint, c_long, c_uchar, c_ulong, c_ushort};
use gio_sys;
use cairo_sys::{cairo_t, cairo_surface_t, cairo_region_t};
use glib_sys::{GType, GError, gboolean, gpointer, GDate, GArray, GList};
use gtypes::{gchar, gint, gdouble, gsize, guint};

type time_t = c_long;

// FIXME: is this the correct way to get opaque types?
// FIXME: alternative: https://docs.rs/cairo-sys-rs/0.5.0/src/cairo_sys/lib.rs.html#64
// NOTE: https://github.com/rust-lang/rust/issues/27303
// NOTE: ask F/O about this
pub enum PopplerDocument {}
pub enum PopplerFormField {}
pub enum PopplerIndexIter {}
pub enum PopplerFontsIter {}
pub enum PopplerLayersIter {}
pub enum PopplerPage {}
pub enum PopplerFontInfo {}
pub enum PopplerLayer {}
pub enum PopplerPSFile {}
pub enum PopplerMovie {}
pub enum PopplerMedia {}
pub enum PopplerAnnot {}
pub enum PopplerAnnotMarkup {}
pub enum PopplerAnnotText {}
pub enum PopplerAnnotTextMarkup {}
pub enum PopplerAnnotFreeText {}
pub enum PopplerAnnotFileAttachment {}
pub enum PopplerAnnotMovie {}
pub enum PopplerAnnotScreen {}
pub enum PopplerAnnotLine {}
pub enum PopplerAnnotCircle {}
pub enum PopplerAnnotSquare {}
pub enum PopplerStructureElement {}
pub enum PopplerStructureElementIter {}
pub enum PopplerTextSpan {}

use self::poppler_action::PopplerAction;
use self::poppler_dest::PopplerDest;
use self::poppler_attachment::PopplerAttachment;
use self::poppler_color::PopplerColor;
use self::poppler_rectangle::PopplerRectangle;
use self::poppler_attachment::PopplerAttachmentSaveFunc;
use self::poppler_annot::PopplerAnnotCalloutLine;
use self::poppler_point::PopplerPoint;
use self::poppler_media::PopplerMediaSaveFunc;
use self::poppler_page::PopplerPageTransition;

#[link(name = "poppler-glib")]
extern "C" {
    pub fn poppler_error_quark() -> glib_sys::GQuark;

    pub fn poppler_get_backend() -> consts::PopplerBackend;
    pub fn poppler_get_version() -> *const c_char;

    pub fn poppler_document_get_type() -> GType;

    pub fn poppler_document_new_from_file(
        uri: *const c_char,
        password: *const c_char,
        error: *mut *mut GError,
    ) -> *mut PopplerDocument;
    pub fn poppler_document_new_from_data(
        data: *mut c_char,
        length: c_int,
        password: *const c_char,
        error: *mut *mut GError,
    ) -> *mut PopplerDocument;
    pub fn poppler_document_new_from_stream(
        stream: *mut gio_sys::GInputStream,
        length: c_long,
        password: *const c_char,
        cancellable: *mut gio_sys::GCancellable,
        error: *mut *mut GError,
    ) -> *mut PopplerDocument;
    pub fn poppler_document_new_from_gfile(
        file: *mut gio_sys::GFile,
        password: *const c_char,
        cancellable: *mut gio_sys::GCancellable,
        error: *mut *mut GError,
    ) -> *mut PopplerDocument;

    pub fn poppler_document_save(
        document: *mut PopplerDocument,
        uri: *const c_char,
        error: *mut *mut GError,
    ) -> gboolean;
    pub fn poppler_document_save_a_copy(
        document: *mut PopplerDocument,
        uri: *const c_char,
        error: *mut *mut GError,
    ) -> gboolean;

    pub fn poppler_document_get_id(
        document: *mut PopplerDocument,
        permanent_id: *mut *mut c_char,
        update_id: *mut *mut c_char,
    ) -> gboolean;
    pub fn poppler_document_get_n_pages(document: *mut PopplerDocument) -> c_int;
    pub fn poppler_document_get_page(
        document: *mut PopplerDocument,
        index: c_int,
    ) -> *mut PopplerPage;
    pub fn poppler_document_get_page_by_label(
        document: *mut PopplerDocument,
        label: *const c_char,
    ) -> *mut PopplerPage;

    pub fn poppler_document_get_pdf_version_string(document: *mut PopplerDocument) -> *mut c_char;
    pub fn poppler_document_get_pdf_version(
        document: *mut PopplerDocument,
        major_version: *mut c_uint,
        minor_version: *mut c_uint,
    );

    pub fn poppler_document_get_title(document: *mut PopplerDocument) -> *mut c_char;
    pub fn poppler_document_set_title(document: *mut PopplerDocument, title: *const c_char);
    pub fn poppler_document_get_author(document: *mut PopplerDocument) -> *mut c_char;
    pub fn poppler_document_set_author(document: *mut PopplerDocument, title: *const c_char);
    pub fn poppler_document_get_subject(document: *mut PopplerDocument) -> *mut c_char;
    pub fn poppler_document_set_subject(document: *mut PopplerDocument, title: *const c_char);
    pub fn poppler_document_get_keywords(document: *mut PopplerDocument) -> *mut c_char;
    pub fn poppler_document_set_keywords(document: *mut PopplerDocument, title: *const c_char);
    pub fn poppler_document_get_creator(document: *mut PopplerDocument) -> *mut c_char;
    pub fn poppler_document_set_creator(document: *mut PopplerDocument, title: *const c_char);
    pub fn poppler_document_get_producer(document: *mut PopplerDocument) -> *mut c_char;
    pub fn poppler_document_set_producer(document: *mut PopplerDocument, title: *const c_char);
    pub fn poppler_document_get_creation_date(document: *mut PopplerDocument) -> time_t;
    pub fn poppler_document_set_creation_date(
        document: *mut PopplerDocument,
        creation_date: time_t,
    );
    pub fn poppler_document_get_modification_date(document: *mut PopplerDocument) -> time_t;
    pub fn poppler_document_set_modification_date(
        document: *mut PopplerDocument,
        modification_date: time_t,
    );

    pub fn poppler_document_is_linearized(document: *mut PopplerDocument) -> gboolean;
    pub fn poppler_document_get_page_layout(document: *mut PopplerDocument) -> consts::PopplerPageLayout;
    pub fn poppler_document_get_page_mode(document: *mut PopplerDocument) -> consts::PopplerPageMode;
    pub fn poppler_document_get_permissions(document: *mut PopplerDocument) -> consts::PopplerPermissions;
    pub fn poppler_document_get_pdf_subtype_string(document: *mut PopplerDocument) -> *mut c_char;
    pub fn poppler_document_get_pdf_subtype(document: *mut PopplerDocument) -> consts::PopplerPDFSubtype;
    pub fn poppler_document_get_pdf_part(document: *mut PopplerDocument) -> consts::PopplerPDFPart;
    pub fn poppler_document_get_pdf_conformance(
        document: *mut PopplerDocument,
    ) -> consts::PopplerPDFConformance;
    pub fn poppler_document_get_metadata(document: *mut PopplerDocument) -> *mut c_char;
    pub fn poppler_document_get_print_scaling(
        document: *mut PopplerDocument,
    ) -> consts::PopplerPrintScaling;
    pub fn poppler_document_get_n_attachments(document: *mut PopplerDocument) -> c_uint;
    pub fn poppler_document_has_attachments(document: *mut PopplerDocument) -> gboolean;
    pub fn poppler_document_get_attachments(document: *mut PopplerDocument) -> *mut glib_sys::GList;
    pub fn poppler_document_find_dest(
        document: *mut PopplerDocument,
        link_name: *const c_char,
    ) -> *mut PopplerDest;
    pub fn poppler_document_get_form_field(
        document: *mut PopplerDocument,
        id: c_int,
    ) -> *mut PopplerFormField;
    pub fn poppler_index_iter_get_type() -> GType;

    pub fn poppler_index_iter_new(document: *mut PopplerDocument) -> *mut PopplerIndexIter;
    pub fn poppler_index_iter_copy(iter: *mut PopplerIndexIter) -> *mut PopplerIndexIter;
    pub fn poppler_index_iter_free(iter: *mut PopplerIndexIter);
    pub fn poppler_index_iter_get_child(parent: *mut PopplerIndexIter) -> *mut PopplerIndexIter;
    pub fn poppler_index_iter_is_open(iter: *mut PopplerIndexIter) -> gboolean;
    pub fn poppler_index_iter_get_action(iter: *mut PopplerIndexIter) -> *mut PopplerAction;
    pub fn poppler_index_iter_next(iter: *mut PopplerIndexIter) -> gboolean;

    pub fn poppler_font_info_get_type() -> GType;
    pub fn poppler_font_info_new(document: *mut PopplerDocument) -> *mut PopplerFontInfo;
    pub fn poppler_font_info_scan(
        font_info: *mut PopplerFontInfo,
        n_pages: c_int,
        iter: *mut *mut PopplerFontsIter,
    ) -> gboolean;
    pub fn poppler_font_info_free(font_info: *mut PopplerFontInfo);

    pub fn poppler_fonts_iter_get_type() -> GType;
    pub fn poppler_fonts_iter_copy(iter: *mut PopplerFontsIter) -> *mut PopplerFontsIter;
    pub fn poppler_fonts_iter_free(iter: *mut PopplerFontsIter);
    pub fn poppler_fonts_iter_get_name(
        iter: *mut PopplerFontsIter,
    ) -> *const c_char;
    pub fn poppler_fonts_iter_get_full_name(
        iter: *mut PopplerFontsIter,
    ) -> *const c_char;
    pub fn poppler_fonts_iter_get_substitute_name(
        iter: *mut PopplerFontsIter,
    ) -> *const c_char;
    pub fn poppler_fonts_iter_get_file_name(
        iter: *mut PopplerFontsIter,
    ) -> *const c_char;
    pub fn poppler_fonts_iter_get_font_type(iter: *mut PopplerFontsIter) -> consts::PopplerFontType;
    pub fn poppler_fonts_iter_get_encoding(
        iter: *mut PopplerFontsIter,
    ) -> *const c_char;
    pub fn poppler_fonts_iter_is_embedded(iter: *mut PopplerFontsIter) -> gboolean;
    pub fn poppler_fonts_iter_is_subset(iter: *mut PopplerFontsIter) -> gboolean;
    pub fn poppler_fonts_iter_next(iter: *mut PopplerFontsIter) -> gboolean;

    pub fn poppler_layers_iter_get_type() -> GType;
    pub fn poppler_layers_iter_new(document: *mut PopplerDocument) -> *mut PopplerLayersIter;
    pub fn poppler_layers_iter_copy(iter: *mut PopplerLayersIter) -> *mut PopplerLayersIter;
    pub fn poppler_layers_iter_free(iter: *mut PopplerLayersIter);
    pub fn poppler_layers_iter_get_child(parent: *mut PopplerLayersIter) -> *mut PopplerLayersIter;
    pub fn poppler_layers_iter_get_title(iter: *mut PopplerLayersIter) -> *mut c_char;
    pub fn poppler_layers_iter_get_layer(iter: *mut PopplerLayersIter) -> *mut PopplerLayer;
    pub fn poppler_layers_iter_next(iter: *mut PopplerLayersIter) -> gboolean;

    pub fn poppler_ps_file_get_type() -> GType;
    pub fn poppler_ps_file_new(
        document: *mut PopplerDocument,
        filename: *const ::std::os::raw::c_char,
        first_page: ::std::os::raw::c_int,
        n_pages: ::std::os::raw::c_int,
    ) -> *mut PopplerPSFile;
    pub fn poppler_ps_file_set_paper_size(ps_file: *mut PopplerPSFile, width: f64, height: f64);
    pub fn poppler_ps_file_set_duplex(ps_file: *mut PopplerPSFile, duplex: gboolean);
    pub fn poppler_ps_file_free(ps_file: *mut PopplerPSFile);

    pub fn poppler_layer_get_type() -> GType;
    pub fn poppler_layer_get_title(layer: *mut PopplerLayer) -> *const c_char;
    pub fn poppler_layer_is_visible(layer: *mut PopplerLayer) -> gboolean;
    pub fn poppler_layer_show(layer: *mut PopplerLayer);
    pub fn poppler_layer_hide(layer: *mut PopplerLayer);
    pub fn poppler_layer_is_parent(layer: *mut PopplerLayer) -> gboolean;
    pub fn poppler_layer_get_radio_button_group_id(layer: *mut PopplerLayer) -> c_int;

    pub fn poppler_action_get_type() -> GType;
    pub fn poppler_action_free(action: *mut PopplerAction);
    pub fn poppler_action_copy(action: *mut PopplerAction) -> *mut PopplerAction;

    pub fn poppler_dest_get_type() -> GType;
    pub fn poppler_dest_free(dest: *mut PopplerDest);
    pub fn poppler_dest_copy(dest: *mut PopplerDest) -> *mut PopplerDest;

    pub fn poppler_named_dest_from_bytestring(
        data: *const c_uchar,
        length: c_ulong,
    ) -> *mut c_char;
    pub fn poppler_named_dest_to_bytestring(
        named_dest: *const c_char,
        length: *mut c_ulong,
    ) -> *mut c_uchar;

    pub fn poppler_form_field_get_type() -> GType;
    pub fn poppler_form_field_get_field_type(field: *mut PopplerFormField) -> consts::PopplerFormFieldType;
    pub fn poppler_form_field_get_id(field: *mut PopplerFormField) -> gint;
    pub fn poppler_form_field_get_font_size(field: *mut PopplerFormField) -> gdouble;
    pub fn poppler_form_field_is_read_only(field: *mut PopplerFormField) -> gboolean;
    pub fn poppler_form_field_get_partial_name(field: *mut PopplerFormField) -> *mut gchar;
    pub fn poppler_form_field_get_mapping_name(field: *mut PopplerFormField) -> *mut gchar;
    pub fn poppler_form_field_get_name(field: *mut PopplerFormField) -> *mut gchar;
    pub fn poppler_form_field_get_action(field: *mut PopplerFormField) -> *mut PopplerAction;
    pub fn poppler_form_field_get_additional_action(
        field: *mut PopplerFormField,
        type_: consts::PopplerAdditionalActionType,
    ) -> *mut PopplerAction;
    pub fn poppler_form_field_button_get_button_type(
        field: *mut PopplerFormField,
    ) -> consts::PopplerFormButtonType;
    pub fn poppler_form_field_button_get_state(field: *mut PopplerFormField) -> gboolean;
    pub fn poppler_form_field_button_set_state(field: *mut PopplerFormField, state: gboolean);
    pub fn poppler_form_field_text_get_text_type(
        field: *mut PopplerFormField,
    ) -> consts::PopplerFormTextType;
    pub fn poppler_form_field_text_get_text(field: *mut PopplerFormField) -> *mut gchar;
    pub fn poppler_form_field_text_set_text(field: *mut PopplerFormField, text: *const gchar);
    pub fn poppler_form_field_text_get_max_len(field: *mut PopplerFormField) -> gint;
    pub fn poppler_form_field_text_do_spell_check(field: *mut PopplerFormField) -> gboolean;
    pub fn poppler_form_field_text_do_scroll(field: *mut PopplerFormField) -> gboolean;
    pub fn poppler_form_field_text_is_rich_text(field: *mut PopplerFormField) -> gboolean;
    pub fn poppler_form_field_text_is_password(field: *mut PopplerFormField) -> gboolean;
    pub fn poppler_form_field_choice_get_choice_type(
        field: *mut PopplerFormField,
    ) -> consts::PopplerFormChoiceType;
    pub fn poppler_form_field_choice_is_editable(field: *mut PopplerFormField) -> gboolean;
    pub fn poppler_form_field_choice_can_select_multiple(field: *mut PopplerFormField) -> gboolean;
    pub fn poppler_form_field_choice_do_spell_check(field: *mut PopplerFormField) -> gboolean;
    pub fn poppler_form_field_choice_commit_on_change(field: *mut PopplerFormField) -> gboolean;
    pub fn poppler_form_field_choice_get_n_items(field: *mut PopplerFormField) -> gint;
    pub fn poppler_form_field_choice_get_item(
        field: *mut PopplerFormField,
        index: gint,
    ) -> *mut gchar;
    pub fn poppler_form_field_choice_is_item_selected(
        field: *mut PopplerFormField,
        index: gint,
    ) -> gboolean;
    pub fn poppler_form_field_choice_select_item(field: *mut PopplerFormField, index: gint);
    pub fn poppler_form_field_choice_unselect_all(field: *mut PopplerFormField);
    pub fn poppler_form_field_choice_toggle_item(field: *mut PopplerFormField, index: gint);
    pub fn poppler_form_field_choice_set_text(field: *mut PopplerFormField, text: *const gchar);
    pub fn poppler_form_field_choice_get_text(field: *mut PopplerFormField) -> *mut gchar;

    pub fn poppler_action_type_get_type() -> GType;
    pub fn poppler_dest_type_get_type() -> GType;
    pub fn poppler_action_movie_operation_get_type() -> GType;
    pub fn poppler_action_layer_action_get_type() -> GType;

    pub fn poppler_annot_type_get_type() -> GType;
    pub fn poppler_annot_flag_get_type() -> GType;
    pub fn poppler_annot_markup_reply_type_get_type() -> GType;
    pub fn poppler_annot_external_data_type_get_type() -> GType;
    pub fn poppler_annot_text_state_get_type() -> GType;
    pub fn poppler_annot_free_text_quadding_get_type() -> GType;

    pub fn poppler_page_layout_get_type() -> GType;
    pub fn poppler_page_mode_get_type() -> GType;
    pub fn poppler_font_type_get_type() -> GType;
    pub fn poppler_viewer_preferences_get_type() -> GType;
    pub fn poppler_print_scaling_get_type() -> GType;
    pub fn poppler_permissions_get_type() -> GType;

    pub fn poppler_pdf_subtype_get_type() -> GType;
    pub fn poppler_pdf_part_get_type() -> GType;
    pub fn poppler_pdf_conformance_get_type() -> GType;

    pub fn poppler_form_field_type_get_type() -> GType;
    pub fn poppler_form_button_type_get_type() -> GType;
    pub fn poppler_form_text_type_get_type() -> GType;
    pub fn poppler_form_choice_type_get_type() -> GType;

    pub fn poppler_additional_action_type_get_type() -> GType;
    pub fn poppler_movie_play_mode_get_type() -> GType;

    pub fn poppler_structure_element_kind_get_type() -> GType;
    pub fn poppler_structure_get_text_flags_get_type() -> GType;
    pub fn poppler_structure_placement_get_type() -> GType;
    pub fn poppler_structure_writing_mode_get_type() -> GType;
    pub fn poppler_structure_border_style_get_type() -> GType;
    pub fn poppler_structure_text_align_get_type() -> GType;
    pub fn poppler_structure_block_align_get_type() -> GType;
    pub fn poppler_structure_inline_align_get_type() -> GType;
    pub fn poppler_structure_text_decoration_get_type() -> GType;
    pub fn poppler_structure_ruby_align_get_type() -> GType;
    pub fn poppler_structure_ruby_position_get_type() -> GType;
    pub fn poppler_structure_glyph_orientation_get_type() -> GType;
    pub fn poppler_structure_list_numbering_get_type() -> GType;
    pub fn poppler_structure_form_role_get_type() -> GType;
    pub fn poppler_structure_form_state_get_type() -> GType;
    pub fn poppler_structure_table_scope_get_type() -> GType;

    pub fn poppler_error_get_type() -> GType;

    pub fn poppler_page_transition_type_get_type() -> GType;
    pub fn poppler_page_transition_alignment_get_type() -> GType;
    pub fn poppler_page_transition_direction_get_type() -> GType;

    pub fn poppler_selection_style_get_type() -> GType;
    pub fn poppler_print_flags_get_type() -> GType;
    pub fn poppler_find_flags_get_type() -> GType;
    pub fn poppler_backend_get_type() -> GType;

    pub fn poppler_attachment_get_type() -> GType;
    pub fn poppler_attachment_save(
        attachment: *mut PopplerAttachment,
        filename: *const ::std::os::raw::c_char,
        error: *mut *mut GError,
    ) -> gboolean;
    pub fn poppler_attachment_save_to_callback(
        attachment: *mut PopplerAttachment,
        save_func: PopplerAttachmentSaveFunc,
        user_data: gpointer,
        error: *mut *mut GError,
    ) -> gboolean;

    pub fn poppler_annot_get_type() -> GType;
    pub fn poppler_annot_get_annot_type(poppler_annot: *mut PopplerAnnot) -> consts::PopplerAnnotType;
    pub fn poppler_annot_get_contents(poppler_annot: *mut PopplerAnnot) -> *mut gchar;
    pub fn poppler_annot_set_contents(poppler_annot: *mut PopplerAnnot, contents: *const gchar);
    pub fn poppler_annot_get_name(poppler_annot: *mut PopplerAnnot) -> *mut gchar;
    pub fn poppler_annot_get_modified(poppler_annot: *mut PopplerAnnot) -> *mut gchar;
    pub fn poppler_annot_get_flags(poppler_annot: *mut PopplerAnnot) -> consts::PopplerAnnotFlag;
    pub fn poppler_annot_set_flags(poppler_annot: *mut PopplerAnnot, flags: consts::PopplerAnnotFlag);
    pub fn poppler_annot_get_color(poppler_annot: *mut PopplerAnnot) -> *mut PopplerColor;
    pub fn poppler_annot_set_color(
        poppler_annot: *mut PopplerAnnot,
        poppler_color: *mut PopplerColor,
    );
    pub fn poppler_annot_get_page_index(poppler_annot: *mut PopplerAnnot) -> gint;
    pub fn poppler_annot_get_rectangle(
        poppler_annot: *mut PopplerAnnot,
        poppler_rect: *mut PopplerRectangle,
    );
    pub fn poppler_annot_set_rectangle(
        poppler_annot: *mut PopplerAnnot,
        poppler_rect: *mut PopplerRectangle,
    );
    pub fn poppler_annot_markup_get_type() -> GType;
    pub fn poppler_annot_markup_get_label(poppler_annot: *mut PopplerAnnotMarkup) -> *mut gchar;
    pub fn poppler_annot_markup_set_label(
        poppler_annot: *mut PopplerAnnotMarkup,
        label: *const gchar,
    );
    pub fn poppler_annot_markup_has_popup(poppler_annot: *mut PopplerAnnotMarkup) -> gboolean;
    pub fn poppler_annot_markup_set_popup(
        poppler_annot: *mut PopplerAnnotMarkup,
        popup_rect: *mut PopplerRectangle,
    );
    pub fn poppler_annot_markup_get_popup_is_open(
        poppler_annot: *mut PopplerAnnotMarkup,
    ) -> gboolean;
    pub fn poppler_annot_markup_set_popup_is_open(
        poppler_annot: *mut PopplerAnnotMarkup,
        is_open: gboolean,
    );
    pub fn poppler_annot_markup_get_popup_rectangle(
        poppler_annot: *mut PopplerAnnotMarkup,
        poppler_rect: *mut PopplerRectangle,
    ) -> gboolean;
    pub fn poppler_annot_markup_set_popup_rectangle(
        poppler_annot: *mut PopplerAnnotMarkup,
        poppler_rect: *mut PopplerRectangle,
    );
    pub fn poppler_annot_markup_get_opacity(poppler_annot: *mut PopplerAnnotMarkup) -> gdouble;
    pub fn poppler_annot_markup_set_opacity(
        poppler_annot: *mut PopplerAnnotMarkup,
        opacity: gdouble,
    );
    pub fn poppler_annot_markup_get_date(poppler_annot: *mut PopplerAnnotMarkup) -> *mut GDate;
    pub fn poppler_annot_markup_get_subject(poppler_annot: *mut PopplerAnnotMarkup) -> *mut gchar;
    pub fn poppler_annot_markup_get_reply_to(
        poppler_annot: *mut PopplerAnnotMarkup,
    ) -> consts::PopplerAnnotMarkupReplyType;
    pub fn poppler_annot_markup_get_external_data(
        poppler_annot: *mut PopplerAnnotMarkup,
    ) -> consts::PopplerAnnotExternalDataType;
    pub fn poppler_annot_text_get_type() -> GType;
    pub fn poppler_annot_text_new(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
    ) -> *mut PopplerAnnot;
    pub fn poppler_annot_text_get_is_open(poppler_annot: *mut PopplerAnnotText) -> gboolean;
    pub fn poppler_annot_text_set_is_open(poppler_annot: *mut PopplerAnnotText, is_open: gboolean);
    pub fn poppler_annot_text_get_icon(poppler_annot: *mut PopplerAnnotText) -> *mut gchar;
    pub fn poppler_annot_text_set_icon(poppler_annot: *mut PopplerAnnotText, icon: *const gchar);
    pub fn poppler_annot_text_get_state(
        poppler_annot: *mut PopplerAnnotText,
    ) -> consts::PopplerAnnotTextState;
    pub fn poppler_annot_text_markup_get_type() -> GType;
    pub fn poppler_annot_text_markup_new_highlight(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
        quadrilaterals: *mut GArray,
    ) -> *mut PopplerAnnot;
    pub fn poppler_annot_text_markup_new_squiggly(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
        quadrilaterals: *mut GArray,
    ) -> *mut PopplerAnnot;
    pub fn poppler_annot_text_markup_new_strikeout(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
        quadrilaterals: *mut GArray,
    ) -> *mut PopplerAnnot;
    pub fn poppler_annot_text_markup_new_underline(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
        quadrilaterals: *mut GArray,
    ) -> *mut PopplerAnnot;
    pub fn poppler_annot_text_markup_set_quadrilaterals(
        poppler_annot: *mut PopplerAnnotTextMarkup,
        quadrilaterals: *mut GArray,
    );
    pub fn poppler_annot_text_markup_get_quadrilaterals(
        poppler_annot: *mut PopplerAnnotTextMarkup,
    ) -> *mut GArray;
    pub fn poppler_annot_free_text_get_type() -> GType;
    pub fn poppler_annot_free_text_get_quadding(
        poppler_annot: *mut PopplerAnnotFreeText,
    ) -> consts::PopplerAnnotFreeTextQuadding;
    pub fn poppler_annot_free_text_get_callout_line(
        poppler_annot: *mut PopplerAnnotFreeText,
    ) -> *mut PopplerAnnotCalloutLine;
    pub fn poppler_annot_file_attachment_get_type() -> GType;
    pub fn poppler_annot_file_attachment_get_attachment(
        poppler_annot: *mut PopplerAnnotFileAttachment,
    ) -> *mut PopplerAttachment;
    pub fn poppler_annot_file_attachment_get_name(
        poppler_annot: *mut PopplerAnnotFileAttachment,
    ) -> *mut gchar;
    pub fn poppler_annot_movie_get_type() -> GType;
    pub fn poppler_annot_movie_get_title(poppler_annot: *mut PopplerAnnotMovie) -> *mut gchar;
    pub fn poppler_annot_movie_get_movie(
        poppler_annot: *mut PopplerAnnotMovie,
    ) -> *mut PopplerMovie;
    pub fn poppler_annot_screen_get_type() -> GType;
    pub fn poppler_annot_screen_get_action(
        poppler_annot: *mut PopplerAnnotScreen,
    ) -> *mut PopplerAction;
    pub fn poppler_annot_line_get_type() -> GType;
    pub fn poppler_annot_line_new(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
        start: *mut PopplerPoint,
        end: *mut PopplerPoint,
    ) -> *mut PopplerAnnot;
    pub fn poppler_annot_line_set_vertices(
        poppler_annot: *mut PopplerAnnotLine,
        start: *mut PopplerPoint,
        end: *mut PopplerPoint,
    );
    pub fn poppler_annot_callout_line_get_type() -> GType;
    pub fn poppler_annot_callout_line_new() -> *mut PopplerAnnotCalloutLine;
    pub fn poppler_annot_callout_line_copy(
        callout: *mut PopplerAnnotCalloutLine,
    ) -> *mut PopplerAnnotCalloutLine;
    pub fn poppler_annot_callout_line_free(callout: *mut PopplerAnnotCalloutLine);
    pub fn poppler_annot_circle_get_type() -> GType;
    pub fn poppler_annot_circle_new(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
    ) -> *mut PopplerAnnot;
    pub fn poppler_annot_circle_set_interior_color(
        poppler_annot: *mut PopplerAnnotCircle,
        poppler_color: *mut PopplerColor,
    );
    pub fn poppler_annot_circle_get_interior_color(
        poppler_annot: *mut PopplerAnnotCircle,
    ) -> *mut PopplerColor;
    pub fn poppler_annot_square_get_type() -> GType;
    pub fn poppler_annot_square_new(
        doc: *mut PopplerDocument,
        rect: *mut PopplerRectangle,
    ) -> *mut PopplerAnnot;
    pub fn poppler_annot_square_set_interior_color(
        poppler_annot: *mut PopplerAnnotSquare,
        poppler_color: *mut PopplerColor,
    );
    pub fn poppler_annot_square_get_interior_color(
        poppler_annot: *mut PopplerAnnotSquare,
    ) -> *mut PopplerColor;

    pub fn poppler_date_parse(date: *const gchar, timet: *mut time_t) -> gboolean;

    pub fn poppler_movie_get_type() -> GType;
    pub fn poppler_movie_get_filename(poppler_movie: *mut PopplerMovie) -> *const gchar;
    pub fn poppler_movie_need_poster(poppler_movie: *mut PopplerMovie) -> gboolean;
    pub fn poppler_movie_show_controls(poppler_movie: *mut PopplerMovie) -> gboolean;
    pub fn poppler_movie_get_play_mode(poppler_movie: *mut PopplerMovie) -> consts::PopplerMoviePlayMode;

    pub fn poppler_media_get_type() -> GType;
    pub fn poppler_media_is_embedded(poppler_media: *mut PopplerMedia) -> gboolean;
    pub fn poppler_media_get_filename(poppler_media: *mut PopplerMedia) -> *const gchar;
    pub fn poppler_media_get_mime_type(poppler_media: *mut PopplerMedia) -> *const gchar;
    pub fn poppler_media_save(
        poppler_media: *mut PopplerMedia,
        filename: *const ::std::os::raw::c_char,
        error: *mut *mut GError,
    ) -> gboolean;
    pub fn poppler_media_save_to_callback(
        poppler_media: *mut PopplerMedia,
        save_func: PopplerMediaSaveFunc,
        user_data: gpointer,
        error: *mut *mut GError,
    ) -> gboolean;


    pub fn poppler_structure_element_get_type() -> GType;
    pub fn poppler_structure_element_get_kind(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> consts::PopplerStructureElementKind;
    pub fn poppler_structure_element_get_page(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> gint;
    pub fn poppler_structure_element_is_content(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> gboolean;
    pub fn poppler_structure_element_is_inline(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> gboolean;
    pub fn poppler_structure_element_is_block(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> gboolean;
    pub fn poppler_structure_element_is_grouping(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> gboolean;
    pub fn poppler_structure_element_get_id(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> *mut gchar;
    pub fn poppler_structure_element_get_title(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> *mut gchar;
    pub fn poppler_structure_element_get_abbreviation(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> *mut gchar;
    pub fn poppler_structure_element_get_language(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> *mut gchar;
    pub fn poppler_structure_element_get_text(
        poppler_structure_element: *mut PopplerStructureElement,
        flags: consts::PopplerStructureGetTextFlags,
    ) -> *mut gchar;
    pub fn poppler_structure_element_get_alt_text(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> *mut gchar;
    pub fn poppler_structure_element_get_actual_text(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> *mut gchar;
    pub fn poppler_structure_element_get_text_spans(
        poppler_structure_element: *mut PopplerStructureElement,
        n_text_spans: *mut guint,
    ) -> *mut *mut PopplerTextSpan;
    pub fn poppler_structure_element_get_placement(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> consts::PopplerStructurePlacement;
    pub fn poppler_structure_element_get_writing_mode(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> consts::PopplerStructureWritingMode;
    pub fn poppler_structure_element_get_background_color(
        poppler_structure_element: *mut PopplerStructureElement,
        color: *mut PopplerColor,
    ) -> gboolean;
    pub fn poppler_structure_element_get_border_color(
        poppler_structure_element: *mut PopplerStructureElement,
        colors: *mut PopplerColor,
    ) -> gboolean;
    pub fn poppler_structure_element_get_border_style(
        poppler_structure_element: *mut PopplerStructureElement,
        border_styles: *mut consts::PopplerStructureBorderStyle,
    );
    pub fn poppler_structure_element_get_border_thickness(
        poppler_structure_element: *mut PopplerStructureElement,
        border_thicknesses: *mut gdouble,
    ) -> gboolean;
    pub fn poppler_structure_element_get_padding(
        poppler_structure_element: *mut PopplerStructureElement,
        paddings: *mut gdouble,
    );
    pub fn poppler_structure_element_get_color(
        poppler_structure_element: *mut PopplerStructureElement,
        color: *mut PopplerColor,
    ) -> gboolean;
    pub fn poppler_structure_element_get_space_before(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> gdouble;
    pub fn poppler_structure_element_get_space_after(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> gdouble;
    pub fn poppler_structure_element_get_start_indent(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> gdouble;
    pub fn poppler_structure_element_get_end_indent(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> gdouble;
    pub fn poppler_structure_element_get_text_indent(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> gdouble;
    pub fn poppler_structure_element_get_text_align(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> consts::PopplerStructureTextAlign;
    pub fn poppler_structure_element_get_bounding_box(
        poppler_structure_element: *mut PopplerStructureElement,
        bounding_box: *mut PopplerRectangle,
    ) -> gboolean;
    pub fn poppler_structure_element_get_width(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> gdouble;
    pub fn poppler_structure_element_get_height(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> gdouble;
    pub fn poppler_structure_element_get_block_align(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> consts::PopplerStructureBlockAlign;
    pub fn poppler_structure_element_get_inline_align(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> consts::PopplerStructureInlineAlign;
    pub fn poppler_structure_element_get_table_border_style(
        poppler_structure_element: *mut PopplerStructureElement,
        border_styles: *mut consts::PopplerStructureBorderStyle,
    );
    pub fn poppler_structure_element_get_table_padding(
        poppler_structure_element: *mut PopplerStructureElement,
        paddings: *mut gdouble,
    );
    pub fn poppler_structure_element_get_baseline_shift(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> gdouble;
    pub fn poppler_structure_element_get_line_height(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> gdouble;
    pub fn poppler_structure_element_get_text_decoration_color(
        poppler_structure_element: *mut PopplerStructureElement,
        color: *mut PopplerColor,
    ) -> gboolean;
    pub fn poppler_structure_element_get_text_decoration_thickness(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> gdouble;
    pub fn poppler_structure_element_get_text_decoration_type(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> consts::PopplerStructureTextDecoration;
    pub fn poppler_structure_element_get_ruby_align(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> consts::PopplerStructureRubyAlign;
    pub fn poppler_structure_element_get_ruby_position(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> consts::PopplerStructureRubyPosition;
    pub fn poppler_structure_element_get_glyph_orientation(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> consts::PopplerStructureGlyphOrientation;
    pub fn poppler_structure_element_get_column_count(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> guint;
    pub fn poppler_structure_element_get_column_gaps(
        poppler_structure_element: *mut PopplerStructureElement,
        n_values: *mut guint,
    ) -> *mut gdouble;
    pub fn poppler_structure_element_get_column_widths(
        poppler_structure_element: *mut PopplerStructureElement,
        n_values: *mut guint,
    ) -> *mut gdouble;
    pub fn poppler_structure_element_get_list_numbering(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> consts::PopplerStructureListNumbering;
    pub fn poppler_structure_element_get_form_role(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> consts::PopplerStructureFormRole;
    pub fn poppler_structure_element_get_form_state(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> consts::PopplerStructureFormState;
    pub fn poppler_structure_element_get_form_description(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> *mut gchar;
    pub fn poppler_structure_element_get_table_row_span(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> guint;
    pub fn poppler_structure_element_get_table_column_span(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> guint;
    pub fn poppler_structure_element_get_table_headers(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> *mut *mut gchar;
    pub fn poppler_structure_element_get_table_scope(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> consts::PopplerStructureTableScope;
    pub fn poppler_structure_element_get_table_summary(
        poppler_structure_element: *mut PopplerStructureElement,
    ) -> *mut gchar;
    pub fn poppler_structure_element_iter_get_type() -> GType;
    pub fn poppler_structure_element_iter_new(
        poppler_document: *mut PopplerDocument,
    ) -> *mut PopplerStructureElementIter;
    pub fn poppler_structure_element_iter_get_child(
        parent: *mut PopplerStructureElementIter,
    ) -> *mut PopplerStructureElementIter;
    pub fn poppler_structure_element_iter_copy(
        iter: *mut PopplerStructureElementIter,
    ) -> *mut PopplerStructureElementIter;
    pub fn poppler_structure_element_iter_get_element(
        iter: *mut PopplerStructureElementIter,
    ) -> *mut PopplerStructureElement;
    pub fn poppler_structure_element_iter_next(iter: *mut PopplerStructureElementIter) -> gboolean;
    pub fn poppler_structure_element_iter_free(iter: *mut PopplerStructureElementIter);

    pub fn poppler_text_span_get_type() -> GType;
    pub fn poppler_text_span_copy(poppler_text_span: *mut PopplerTextSpan) -> *mut PopplerTextSpan;
    pub fn poppler_text_span_free(poppler_text_span: *mut PopplerTextSpan);
    pub fn poppler_text_span_is_fixed_width_font(
        poppler_text_span: *mut PopplerTextSpan,
    ) -> gboolean;
    pub fn poppler_text_span_is_serif_font(poppler_text_span: *mut PopplerTextSpan) -> gboolean;
    pub fn poppler_text_span_is_bold_font(poppler_text_span: *mut PopplerTextSpan) -> gboolean;
    pub fn poppler_text_span_get_color(
        poppler_text_span: *mut PopplerTextSpan,
        color: *mut PopplerColor,
    );
    pub fn poppler_text_span_get_text(poppler_text_span: *mut PopplerTextSpan) -> *const gchar;
    pub fn poppler_text_span_get_font_name(poppler_text_span: *mut PopplerTextSpan)
        -> *const gchar;
    pub fn cairo_version() -> ::std::os::raw::c_int;
    pub fn cairo_version_string() -> *const ::std::os::raw::c_char;

    pub fn poppler_page_get_type() -> GType;
    pub fn poppler_page_render(page: *mut PopplerPage, cairo: *mut cairo_t);
    pub fn poppler_page_render_for_printing(page: *mut PopplerPage, cairo: *mut cairo_t);
    pub fn poppler_page_render_for_printing_with_options(
        page: *mut PopplerPage,
        cairo: *mut cairo_t,
        options: consts::PopplerPrintFlags,
    );
    pub fn poppler_page_get_thumbnail(page: *mut PopplerPage) -> *mut cairo_surface_t;
    pub fn poppler_page_render_selection(
        page: *mut PopplerPage,
        cairo: *mut cairo_t,
        selection: *mut PopplerRectangle,
        old_selection: *mut PopplerRectangle,
        style: consts::PopplerSelectionStyle,
        glyph_color: *mut PopplerColor,
        background_color: *mut PopplerColor,
    );
    pub fn poppler_page_get_size(page: *mut PopplerPage, width: *mut f64, height: *mut f64);
    pub fn poppler_page_get_index(page: *mut PopplerPage) -> ::std::os::raw::c_int;
    pub fn poppler_page_get_label(page: *mut PopplerPage) -> *mut gchar;
    pub fn poppler_page_get_duration(page: *mut PopplerPage) -> f64;
    pub fn poppler_page_get_transition(page: *mut PopplerPage) -> *mut PopplerPageTransition;
    pub fn poppler_page_get_thumbnail_size(
        page: *mut PopplerPage,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
    ) -> gboolean;
    pub fn poppler_page_find_text_with_options(
        page: *mut PopplerPage,
        text: *const ::std::os::raw::c_char,
        options: consts::PopplerFindFlags,
    ) -> *mut GList;
    pub fn poppler_page_find_text(
        page: *mut PopplerPage,
        text: *const ::std::os::raw::c_char,
    ) -> *mut GList;
    pub fn poppler_page_render_to_ps(page: *mut PopplerPage, ps_file: *mut PopplerPSFile);
    pub fn poppler_page_get_text(page: *mut PopplerPage) -> *mut ::std::os::raw::c_char;
    pub fn poppler_page_get_text_for_area(
        page: *mut PopplerPage,
        area: *mut PopplerRectangle,
    ) -> *mut ::std::os::raw::c_char;
    pub fn poppler_page_get_selected_text(
        page: *mut PopplerPage,
        style: consts::PopplerSelectionStyle,
        selection: *mut PopplerRectangle,
    ) -> *mut ::std::os::raw::c_char;
    pub fn poppler_page_get_selected_region(
        page: *mut PopplerPage,
        scale: gdouble,
        style: consts::PopplerSelectionStyle,
        selection: *mut PopplerRectangle,
    ) -> *mut cairo_region_t;
    pub fn poppler_page_get_selection_region(
        page: *mut PopplerPage,
        scale: gdouble,
        style: consts::PopplerSelectionStyle,
        selection: *mut PopplerRectangle,
    ) -> *mut GList;
    pub fn poppler_page_selection_region_free(region: *mut GList);
    pub fn poppler_page_get_link_mapping(page: *mut PopplerPage) -> *mut GList;
    pub fn poppler_page_free_link_mapping(list: *mut GList);
    pub fn poppler_page_get_image_mapping(page: *mut PopplerPage) -> *mut GList;
    pub fn poppler_page_free_image_mapping(list: *mut GList);
    pub fn poppler_page_get_image(page: *mut PopplerPage, image_id: gint) -> *mut cairo_surface_t;
    pub fn poppler_page_get_form_field_mapping(page: *mut PopplerPage) -> *mut GList;
    pub fn poppler_page_free_form_field_mapping(list: *mut GList);
    pub fn poppler_page_get_annot_mapping(page: *mut PopplerPage) -> *mut GList;
    pub fn poppler_page_free_annot_mapping(list: *mut GList);
    pub fn poppler_page_add_annot(page: *mut PopplerPage, annot: *mut PopplerAnnot);
    pub fn poppler_page_remove_annot(page: *mut PopplerPage, annot: *mut PopplerAnnot);
    pub fn poppler_page_get_crop_box(page: *mut PopplerPage, rect: *mut PopplerRectangle);
    pub fn poppler_page_get_text_layout(
        page: *mut PopplerPage,
        rectangles: *mut *mut PopplerRectangle,
        n_rectangles: *mut guint,
    ) -> gboolean;
    pub fn poppler_page_get_text_layout_for_area(
        page: *mut PopplerPage,
        area: *mut PopplerRectangle,
        rectangles: *mut *mut PopplerRectangle,
        n_rectangles: *mut guint,
    ) -> gboolean;
    pub fn poppler_page_get_text_attributes(page: *mut PopplerPage) -> *mut GList;
    pub fn poppler_page_free_text_attributes(list: *mut GList);
    pub fn poppler_page_get_text_attributes_for_area(
        page: *mut PopplerPage,
        area: *mut PopplerRectangle,
    ) -> *mut GList;
}
