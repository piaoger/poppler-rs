#![allow(dead_code, non_upper_case_globals)]

pub const PopplerError_POPPLER_ERROR_INVALID: PopplerError = 0;
pub const PopplerError_POPPLER_ERROR_ENCRYPTED: PopplerError = 1;
pub const PopplerError_POPPLER_ERROR_OPEN_FILE: PopplerError = 2;
pub const PopplerError_POPPLER_ERROR_BAD_CATALOG: PopplerError = 3;
pub const PopplerError_POPPLER_ERROR_DAMAGED: PopplerError = 4;
/// PopplerError:
/// @POPPLER_ERROR_INVALID: Generic error when a document operation fails
/// @POPPLER_ERROR_ENCRYPTED: Document is encrypted
/// @POPPLER_ERROR_OPEN_FILE: File could not be opened for writing when saving document
/// @POPPLER_ERROR_BAD_CATALOG: Failed to read the document catalog
/// @POPPLER_ERROR_DAMAGED: Document is damaged
///
/// Error codes returned by #PopplerDocument
pub type PopplerError = u32;




pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_REPLACE: PopplerPageTransitionType = 0;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_SPLIT: PopplerPageTransitionType = 1;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_BLINDS: PopplerPageTransitionType = 2;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_BOX: PopplerPageTransitionType = 3;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_WIPE: PopplerPageTransitionType = 4;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_DISSOLVE: PopplerPageTransitionType = 5;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_GLITTER: PopplerPageTransitionType = 6;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_FLY: PopplerPageTransitionType = 7;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_PUSH: PopplerPageTransitionType = 8;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_COVER: PopplerPageTransitionType = 9;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_UNCOVER: PopplerPageTransitionType = 10;
pub const PopplerPageTransitionType_POPPLER_PAGE_TRANSITION_FADE: PopplerPageTransitionType = 11;
/// PopplerPageTransitionType:
/// @POPPLER_PAGE_TRANSITION_REPLACE: the new page replace the old one
/// @POPPLER_PAGE_TRANSITION_SPLIT: two lines sweep across the screen, revealing the new page
/// @POPPLER_PAGE_TRANSITION_BLINDS: multiple lines, evenly spaced across the screen, synchronously
/// sweep in the same direction to reveal the new page
/// @POPPLER_PAGE_TRANSITION_BOX: a rectangular box sweeps inward from the edges of the page or
/// outward from the center revealing the new page
/// @POPPLER_PAGE_TRANSITION_WIPE: a single line sweeps across the screen from one edge to the other
/// revealing the new page
/// @POPPLER_PAGE_TRANSITION_DISSOLVE: the old page dissolves gradually to reveal the new one
/// @POPPLER_PAGE_TRANSITION_GLITTER: similar to #POPPLER_PAGE_TRANSITION_DISSOLVE, except that the effect
/// sweeps across the page in a wide band moving from one side of the screen to the other
/// @POPPLER_PAGE_TRANSITION_FLY: changes are flown out or in to or from a location that is offscreen
/// @POPPLER_PAGE_TRANSITION_PUSH: the old page slides off the screen while the new page slides in
/// @POPPLER_PAGE_TRANSITION_COVER: the new page slides on to the screen covering the old page
/// @POPPLER_PAGE_TRANSITION_UNCOVER: the old page slides off the screen uncovering the new page
/// @POPPLER_PAGE_TRANSITION_FADE: the new page gradually becomes visible through the old one
///
/// Page transition types
pub type PopplerPageTransitionType = u32;




pub const PopplerPageTransitionAlignment_POPPLER_PAGE_TRANSITION_HORIZONTAL:
    PopplerPageTransitionAlignment = 0;
pub const PopplerPageTransitionAlignment_POPPLER_PAGE_TRANSITION_VERTICAL:
    PopplerPageTransitionAlignment = 1;
/// PopplerPageTransitionAlignment:
/// @POPPLER_PAGE_TRANSITION_HORIZONTAL: horizontal dimension
/// @POPPLER_PAGE_TRANSITION_VERTICAL: vertical dimension
///
/// Page transition alignment types for #POPPLER_PAGE_TRANSITION_SPLIT
/// and #POPPLER_PAGE_TRANSITION_BLINDS transition types
pub type PopplerPageTransitionAlignment = u32;




pub const PopplerPageTransitionDirection_POPPLER_PAGE_TRANSITION_INWARD:
    PopplerPageTransitionDirection = 0;
pub const PopplerPageTransitionDirection_POPPLER_PAGE_TRANSITION_OUTWARD:
    PopplerPageTransitionDirection = 1;
/// PopplerPageTransitionDirection:
/// @POPPLER_PAGE_TRANSITION_INWARD: inward from the edges of the page
/// @POPPLER_PAGE_TRANSITION_OUTWARD: outward from the center of the page
///
/// Page transition direction types for #POPPLER_PAGE_TRANSITION_SPLIT,
/// #POPPLER_PAGE_TRANSITION_BOX and #POPPLER_PAGE_TRANSITION_FLY transition types
pub type PopplerPageTransitionDirection = u32;




pub const PopplerSelectionStyle_POPPLER_SELECTION_GLYPH: PopplerSelectionStyle = 0;
pub const PopplerSelectionStyle_POPPLER_SELECTION_WORD: PopplerSelectionStyle = 1;
pub const PopplerSelectionStyle_POPPLER_SELECTION_LINE: PopplerSelectionStyle = 2;
/// PopplerSelectionStyle:
/// @POPPLER_SELECTION_GLYPH: glyph is the minimum unit for selection
/// @POPPLER_SELECTION_WORD: word is the minimum unit for selection
/// @POPPLER_SELECTION_LINE: line is the minimum unit for selection
///
/// Selection styles
pub type PopplerSelectionStyle = u32;




pub const PopplerPrintFlags_POPPLER_PRINT_DOCUMENT: PopplerPrintFlags = 0;
pub const PopplerPrintFlags_POPPLER_PRINT_MARKUP_ANNOTS: PopplerPrintFlags = 1;
pub const PopplerPrintFlags_POPPLER_PRINT_STAMP_ANNOTS_ONLY: PopplerPrintFlags = 2;
pub const PopplerPrintFlags_POPPLER_PRINT_ALL: PopplerPrintFlags = 1;
/// PopplerPrintFlags:
/// @POPPLER_PRINT_DOCUMENT: print main document contents
/// @POPPLER_PRINT_MARKUP_ANNOTS: print document and markup annotations
/// @POPPLER_PRINT_STAMP_ANNOTS_ONLY: print document and only stamp annotations
/// @POPPLER_PRINT_ALL: print main document contents and all markup annotations
///
/// Printing flags
///
/// Since: 0.16
pub type PopplerPrintFlags = u32;




pub const PopplerFindFlags_POPPLER_FIND_DEFAULT: PopplerFindFlags = 0;
pub const PopplerFindFlags_POPPLER_FIND_CASE_SENSITIVE: PopplerFindFlags = 1;
pub const PopplerFindFlags_POPPLER_FIND_BACKWARDS: PopplerFindFlags = 2;
pub const PopplerFindFlags_POPPLER_FIND_WHOLE_WORDS_ONLY: PopplerFindFlags = 4;
pub const PopplerFindFlags_POPPLER_FIND_IGNORE_DIACRITICS: PopplerFindFlags = 8;
/// PopplerFindFlags:
/// @POPPLER_FIND_DEFAULT: use default search settings
/// @POPPLER_FIND_CASE_SENSITIVE: do case sensitive search
/// @POPPLER_FIND_BACKWARDS: search backwards
/// @POPPLER_FIND_WHOLE_WORDS_ONLY: search only whole words
/// @POPPLER_FIND_IGNORE_DIACRITICS: do diacritics insensitive search,
/// i.e. ignore accents, umlauts, diaeresis,etc. while matching. This
/// option will be ignored if the search term is not pure ascii. Since 0.73.
///
/// Flags using while searching text in a page
///
/// Since: 0.22
pub type PopplerFindFlags = u32;


pub const PopplerBackend_POPPLER_BACKEND_UNKNOWN: PopplerBackend = 0;
pub const PopplerBackend_POPPLER_BACKEND_SPLASH: PopplerBackend = 1;
pub const PopplerBackend_POPPLER_BACKEND_CAIRO: PopplerBackend = 2;
/// PopplerBackend:
/// @POPPLER_BACKEND_UNKNOWN: Unknown backend
/// @POPPLER_BACKEND_SPLASH: Splash backend
/// @POPPLER_BACKEND_CAIRO: Cairo backend
///
/// Backend codes returned by poppler_get_backend().
pub type PopplerBackend = u32;



pub const PopplerPageLayout_POPPLER_PAGE_LAYOUT_UNSET: PopplerPageLayout = 0;
pub const PopplerPageLayout_POPPLER_PAGE_LAYOUT_SINGLE_PAGE: PopplerPageLayout = 1;
pub const PopplerPageLayout_POPPLER_PAGE_LAYOUT_ONE_COLUMN: PopplerPageLayout = 2;
pub const PopplerPageLayout_POPPLER_PAGE_LAYOUT_TWO_COLUMN_LEFT: PopplerPageLayout = 3;
pub const PopplerPageLayout_POPPLER_PAGE_LAYOUT_TWO_COLUMN_RIGHT: PopplerPageLayout = 4;
pub const PopplerPageLayout_POPPLER_PAGE_LAYOUT_TWO_PAGE_LEFT: PopplerPageLayout = 5;
pub const PopplerPageLayout_POPPLER_PAGE_LAYOUT_TWO_PAGE_RIGHT: PopplerPageLayout = 6;
/// PopplerPageLayout:
/// @POPPLER_PAGE_LAYOUT_UNSET: no specific layout set
/// @POPPLER_PAGE_LAYOUT_SINGLE_PAGE: one page at a time
/// @POPPLER_PAGE_LAYOUT_ONE_COLUMN: pages in one column
/// @POPPLER_PAGE_LAYOUT_TWO_COLUMN_LEFT: pages in two columns with odd numbered pages on the left
/// @POPPLER_PAGE_LAYOUT_TWO_COLUMN_RIGHT: pages in two columns with odd numbered pages on the right
/// @POPPLER_PAGE_LAYOUT_TWO_PAGE_LEFT: two pages at a time with odd numbered pages on the left
/// @POPPLER_PAGE_LAYOUT_TWO_PAGE_RIGHT: two pages at a time with odd numbered pages on the right
///
/// Page layout types
pub type PopplerPageLayout = u32;




pub const PopplerPageMode_POPPLER_PAGE_MODE_UNSET: PopplerPageMode = 0;
pub const PopplerPageMode_POPPLER_PAGE_MODE_NONE: PopplerPageMode = 1;
pub const PopplerPageMode_POPPLER_PAGE_MODE_USE_OUTLINES: PopplerPageMode = 2;
pub const PopplerPageMode_POPPLER_PAGE_MODE_USE_THUMBS: PopplerPageMode = 3;
pub const PopplerPageMode_POPPLER_PAGE_MODE_FULL_SCREEN: PopplerPageMode = 4;
pub const PopplerPageMode_POPPLER_PAGE_MODE_USE_OC: PopplerPageMode = 5;
pub const PopplerPageMode_POPPLER_PAGE_MODE_USE_ATTACHMENTS: PopplerPageMode = 6;
/// PopplerPageMode:
/// @POPPLER_PAGE_MODE_UNSET: no specific mode set
/// @POPPLER_PAGE_MODE_NONE: neither document outline nor thumbnails visible
/// @POPPLER_PAGE_MODE_USE_OUTLINES: document outline visible
/// @POPPLER_PAGE_MODE_USE_THUMBS: thumbnails visible
/// @POPPLER_PAGE_MODE_FULL_SCREEN: full-screen mode
/// @POPPLER_PAGE_MODE_USE_OC: layers panel visible
/// @POPPLER_PAGE_MODE_USE_ATTACHMENTS: attachments panel visible
///
/// Page modes
pub type PopplerPageMode = u32;




pub const PopplerFontType_POPPLER_FONT_TYPE_UNKNOWN: PopplerFontType = 0;
pub const PopplerFontType_POPPLER_FONT_TYPE_TYPE1: PopplerFontType = 1;
pub const PopplerFontType_POPPLER_FONT_TYPE_TYPE1C: PopplerFontType = 2;
pub const PopplerFontType_POPPLER_FONT_TYPE_TYPE1COT: PopplerFontType = 3;
pub const PopplerFontType_POPPLER_FONT_TYPE_TYPE3: PopplerFontType = 4;
pub const PopplerFontType_POPPLER_FONT_TYPE_TRUETYPE: PopplerFontType = 5;
pub const PopplerFontType_POPPLER_FONT_TYPE_TRUETYPEOT: PopplerFontType = 6;
pub const PopplerFontType_POPPLER_FONT_TYPE_CID_TYPE0: PopplerFontType = 7;
pub const PopplerFontType_POPPLER_FONT_TYPE_CID_TYPE0C: PopplerFontType = 8;
pub const PopplerFontType_POPPLER_FONT_TYPE_CID_TYPE0COT: PopplerFontType = 9;
pub const PopplerFontType_POPPLER_FONT_TYPE_CID_TYPE2: PopplerFontType = 10;
pub const PopplerFontType_POPPLER_FONT_TYPE_CID_TYPE2OT: PopplerFontType = 11;
/// PopplerFontType:
/// @POPPLER_FONT_TYPE_UNKNOWN: unknown font type
/// @POPPLER_FONT_TYPE_TYPE1: Type 1 font type
/// @POPPLER_FONT_TYPE_TYPE1C: Type 1 font type embedded in Compact Font Format (CFF) font program
/// @POPPLER_FONT_TYPE_TYPE1COT: Type 1 font type embedded in OpenType font program
/// @POPPLER_FONT_TYPE_TYPE3: A font type that is defined with PDF graphics operators
/// @POPPLER_FONT_TYPE_TRUETYPE: TrueType font type
/// @POPPLER_FONT_TYPE_TRUETYPEOT: TrueType font type embedded in OpenType font program
/// @POPPLER_FONT_TYPE_CID_TYPE0: CIDFont type based on Type 1 font technology
/// @POPPLER_FONT_TYPE_CID_TYPE0C: CIDFont type based on Type 1 font technology embedded in CFF font program
/// @POPPLER_FONT_TYPE_CID_TYPE0COT: CIDFont type based on Type 1 font technology embedded in OpenType font program
/// @POPPLER_FONT_TYPE_CID_TYPE2: CIDFont type based on TrueType font technology
/// @POPPLER_FONT_TYPE_CID_TYPE2OT: CIDFont type based on TrueType font technology embedded in OpenType font program
///
/// Font types
pub type PopplerFontType = u32;




pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_UNSET: PopplerViewerPreferences = 0;
pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_HIDE_TOOLBAR:
    PopplerViewerPreferences = 1;
pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_HIDE_MENUBAR:
    PopplerViewerPreferences = 2;
pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_HIDE_WINDOWUI:
    PopplerViewerPreferences = 4;
pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_FIT_WINDOW: PopplerViewerPreferences =
    8;
pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_CENTER_WINDOW:
    PopplerViewerPreferences = 16;
pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_DISPLAY_DOC_TITLE:
    PopplerViewerPreferences = 32;
pub const PopplerViewerPreferences_POPPLER_VIEWER_PREFERENCES_DIRECTION_RTL:
    PopplerViewerPreferences = 64;
/// PopplerViewerPreferences:
/// @POPPLER_VIEWER_PREFERENCES_UNSET: no preferences set
/// @POPPLER_VIEWER_PREFERENCES_HIDE_TOOLBAR: hider toolbars when document is active
/// @POPPLER_VIEWER_PREFERENCES_HIDE_MENUBAR: hide menu bar when document is active
/// @POPPLER_VIEWER_PREFERENCES_HIDE_WINDOWUI: hide UI elements in document's window
/// @POPPLER_VIEWER_PREFERENCES_FIT_WINDOW: resize document's window to fit the size of the first displayed page
/// @POPPLER_VIEWER_PREFERENCES_CENTER_WINDOW: position the document's window in the center of the screen
/// @POPPLER_VIEWER_PREFERENCES_DISPLAY_DOC_TITLE: display document title in window's title bar
/// @POPPLER_VIEWER_PREFERENCES_DIRECTION_RTL: the predominant reading order for text is right to left
///
/// Viewer preferences"
pub type PopplerViewerPreferences = u32;




pub const PopplerPrintScaling_POPPLER_PRINT_SCALING_APP_DEFAULT: PopplerPrintScaling = 0;
pub const PopplerPrintScaling_POPPLER_PRINT_SCALING_NONE: PopplerPrintScaling = 1;
/// PopplerPrintScaling:
/// @POPPLER_PRINT_SCALING_APP_DEFAULT: application's default page scaling
/// @POPPLER_PRINT_SCALING_NONE: no page scaling
///
/// PrintScaling viewer preference
///
/// Since: 0.73
pub type PopplerPrintScaling = u32;




pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_PRINT: PopplerPermissions = 1;
pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_MODIFY: PopplerPermissions = 2;
pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_COPY: PopplerPermissions = 4;
pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_ADD_NOTES: PopplerPermissions = 8;
pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_FILL_FORM: PopplerPermissions = 16;
pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_EXTRACT_CONTENTS: PopplerPermissions = 32;
pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_ASSEMBLE: PopplerPermissions = 64;
pub const PopplerPermissions_POPPLER_PERMISSIONS_OK_TO_PRINT_HIGH_RESOLUTION: PopplerPermissions =
    128;
pub const PopplerPermissions_POPPLER_PERMISSIONS_FULL: PopplerPermissions = 255;
/// PopplerPermissions:
/// @POPPLER_PERMISSIONS_OK_TO_PRINT: document can be printer
/// @POPPLER_PERMISSIONS_OK_TO_MODIFY: document contents can be modified
/// @POPPLER_PERMISSIONS_OK_TO_COPY: document can be copied
/// @POPPLER_PERMISSIONS_OK_TO_ADD_NOTES: annotations can added to the document
/// @POPPLER_PERMISSIONS_OK_TO_FILL_FORM: interactive form fields can be filled in
/// @POPPLER_PERMISSIONS_OK_TO_EXTRACT_CONTENTS: extract text and graphics
/// (in support of accessibility to users with disabilities or for other purposes). Since 0.18
/// @POPPLER_PERMISSIONS_OK_TO_ASSEMBLE: assemble the document (insert, rotate, or delete pages and create
/// bookmarks or thumbnail images). Since 0.18
/// @POPPLER_PERMISSIONS_OK_TO_PRINT_HIGH_RESOLUTION: document can be printer at high resolution. Since 0.18
/// @POPPLER_PERMISSIONS_FULL: document permits all operations
///
/// Permissions
pub type PopplerPermissions = u32;




pub const PopplerPDFSubtype_POPPLER_PDF_SUBTYPE_UNSET: PopplerPDFSubtype = 0;
pub const PopplerPDFSubtype_POPPLER_PDF_SUBTYPE_PDF_A: PopplerPDFSubtype = 1;
pub const PopplerPDFSubtype_POPPLER_PDF_SUBTYPE_PDF_E: PopplerPDFSubtype = 2;
pub const PopplerPDFSubtype_POPPLER_PDF_SUBTYPE_PDF_UA: PopplerPDFSubtype = 3;
pub const PopplerPDFSubtype_POPPLER_PDF_SUBTYPE_PDF_VT: PopplerPDFSubtype = 4;
pub const PopplerPDFSubtype_POPPLER_PDF_SUBTYPE_PDF_X: PopplerPDFSubtype = 5;
pub const PopplerPDFSubtype_POPPLER_PDF_SUBTYPE_NONE: PopplerPDFSubtype = 6;
/// PopplerPDFSubtype:
/// @POPPLER_PDF_SUBTYPE_UNSET: Null
/// @POPPLER_PDF_SUBTYPE_PDF_A: ISO 19005 - Document management -- Electronic document file format for long-term preservation (PDF/A)
/// @POPPLER_PDF_SUBTYPE_PDF_E: ISO 24517 - Document management -- Engineering document format using PDF (PDF/E)
/// @POPPLER_PDF_SUBTYPE_PDF_UA: ISO 14289 - Document management applications -- Electronic document file format enhancement for accessibility (PDF/UA)
/// @POPPLER_PDF_SUBTYPE_PDF_VT: ISO 16612 - Graphic technology -- Variable data exchange (PDF/VT)
/// @POPPLER_PDF_SUBTYPE_PDF_X: ISO 15930 - Graphic technology -- Prepress digital data exchange (PDF/X)
/// @POPPLER_PDF_SUBTYPE_NONE: Not compliant with the above standards
///
/// PDF Subtype
///
/// Since: 0.70
pub type PopplerPDFSubtype = u32;




pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_UNSET: PopplerPDFPart = 0;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_1: PopplerPDFPart = 1;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_2: PopplerPDFPart = 2;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_3: PopplerPDFPart = 3;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_4: PopplerPDFPart = 4;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_5: PopplerPDFPart = 5;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_6: PopplerPDFPart = 6;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_7: PopplerPDFPart = 7;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_8: PopplerPDFPart = 8;
pub const PopplerPDFPart_POPPLER_PDF_SUBTYPE_PART_NONE: PopplerPDFPart = 9;
/// PopplerPDFPart:
/// @POPPLER_PDF_SUBTYPE_PART_UNSET: Null
/// @POPPLER_PDF_SUBTYPE_PART_1: 1
/// @POPPLER_PDF_SUBTYPE_PART_2: 2
/// @POPPLER_PDF_SUBTYPE_PART_3: 3
/// @POPPLER_PDF_SUBTYPE_PART_4: 4
/// @POPPLER_PDF_SUBTYPE_PART_5: 5
/// @POPPLER_PDF_SUBTYPE_PART_6: 6
/// @POPPLER_PDF_SUBTYPE_PART_7: 7
/// @POPPLER_PDF_SUBTYPE_PART_8: 8
/// @POPPLER_PDF_SUBTYPE_PART_NONE: No part available
///
/// PDF Subtype Part
///
/// Since: 0.70
pub type PopplerPDFPart = u32;




pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_UNSET: PopplerPDFConformance = 0;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_A: PopplerPDFConformance = 1;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_B: PopplerPDFConformance = 2;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_G: PopplerPDFConformance = 3;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_N: PopplerPDFConformance = 4;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_P: PopplerPDFConformance = 5;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_PG: PopplerPDFConformance = 6;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_U: PopplerPDFConformance = 7;
pub const PopplerPDFConformance_POPPLER_PDF_SUBTYPE_CONF_NONE: PopplerPDFConformance = 8;
/// PopplerPDFConformance:
/// @POPPLER_PDF_SUBTYPE_CONF_UNSET: Null
/// @POPPLER_PDF_SUBTYPE_CONF_A: Level A (accessible) conformance (PDF/A)
/// @POPPLER_PDF_SUBTYPE_CONF_B: Level B (basic) conformance (PDF/A)
/// @POPPLER_PDF_SUBTYPE_CONF_G: Level G (external graphical content) (PDF/X)
/// @POPPLER_PDF_SUBTYPE_CONF_N: Level N (external ICC Profile) (PDF/X)
/// @POPPLER_PDF_SUBTYPE_CONF_P: Level P (ICC Profile) (PDF/X)
/// @POPPLER_PDF_SUBTYPE_CONF_PG: Level PG (conjunction of P and G) (PDF/X)
/// @POPPLER_PDF_SUBTYPE_CONF_U: Level U (Unicode) conformance (PDF/A)
/// @POPPLER_PDF_SUBTYPE_CONF_NONE: No conformance level available
///
/// PDF Subtype Conformance
///
/// Since: 0.70
pub type PopplerPDFConformance = u32;



pub const PopplerFormFieldType_POPPLER_FORM_FIELD_UNKNOWN: PopplerFormFieldType = 0;
pub const PopplerFormFieldType_POPPLER_FORM_FIELD_BUTTON: PopplerFormFieldType = 1;
pub const PopplerFormFieldType_POPPLER_FORM_FIELD_TEXT: PopplerFormFieldType = 2;
pub const PopplerFormFieldType_POPPLER_FORM_FIELD_CHOICE: PopplerFormFieldType = 3;
pub const PopplerFormFieldType_POPPLER_FORM_FIELD_SIGNATURE: PopplerFormFieldType = 4;
pub type PopplerFormFieldType = u32;




pub const PopplerFormButtonType_POPPLER_FORM_BUTTON_PUSH: PopplerFormButtonType = 0;
pub const PopplerFormButtonType_POPPLER_FORM_BUTTON_CHECK: PopplerFormButtonType = 1;
pub const PopplerFormButtonType_POPPLER_FORM_BUTTON_RADIO: PopplerFormButtonType = 2;
pub type PopplerFormButtonType = u32;



pub const PopplerFormTextType_POPPLER_FORM_TEXT_NORMAL: PopplerFormTextType = 0;
pub const PopplerFormTextType_POPPLER_FORM_TEXT_MULTILINE: PopplerFormTextType = 1;
pub const PopplerFormTextType_POPPLER_FORM_TEXT_FILE_SELECT: PopplerFormTextType = 2;
pub type PopplerFormTextType = u32;



pub const PopplerFormChoiceType_POPPLER_FORM_CHOICE_COMBO: PopplerFormChoiceType = 0;
pub const PopplerFormChoiceType_POPPLER_FORM_CHOICE_LIST: PopplerFormChoiceType = 1;
pub type PopplerFormChoiceType = u32;



pub const PopplerAdditionalActionType_POPPLER_ADDITIONAL_ACTION_FIELD_MODIFIED:
    PopplerAdditionalActionType = 0;
pub const PopplerAdditionalActionType_POPPLER_ADDITIONAL_ACTION_FORMAT_FIELD:
    PopplerAdditionalActionType = 1;
pub const PopplerAdditionalActionType_POPPLER_ADDITIONAL_ACTION_VALIDATE_FIELD:
    PopplerAdditionalActionType = 2;
pub const PopplerAdditionalActionType_POPPLER_ADDITIONAL_ACTION_CALCULATE_FIELD:
    PopplerAdditionalActionType = 3;
/// PopplerAdditionalActionType:
/// @POPPLER_ADDITIONAL_ACTION_FIELD_MODIFIED: The action to be performed when the user modifies the field.
/// @POPPLER_ADDITIONAL_ACTION_FORMAT_FIELD: The action to be performed before the field is formatted to
/// display its value.
/// @POPPLER_ADDITIONAL_ACTION_VALIDATE_FIELD: The action to be performed when the field value changes.
/// @POPPLER_ADDITIONAL_ACTION_CALCULATE_FIELD: The action to be performed when the field needs to be
/// recalculated.
///
/// Form field additional action types to be passed to @poppler_form_field_get_additional_action
///
/// Since: 0.72
pub type PopplerAdditionalActionType = u32;



pub const PopplerAnnotType_POPPLER_ANNOT_UNKNOWN: PopplerAnnotType = 0;
pub const PopplerAnnotType_POPPLER_ANNOT_TEXT: PopplerAnnotType = 1;
pub const PopplerAnnotType_POPPLER_ANNOT_LINK: PopplerAnnotType = 2;
pub const PopplerAnnotType_POPPLER_ANNOT_FREE_TEXT: PopplerAnnotType = 3;
pub const PopplerAnnotType_POPPLER_ANNOT_LINE: PopplerAnnotType = 4;
pub const PopplerAnnotType_POPPLER_ANNOT_SQUARE: PopplerAnnotType = 5;
pub const PopplerAnnotType_POPPLER_ANNOT_CIRCLE: PopplerAnnotType = 6;
pub const PopplerAnnotType_POPPLER_ANNOT_POLYGON: PopplerAnnotType = 7;
pub const PopplerAnnotType_POPPLER_ANNOT_POLY_LINE: PopplerAnnotType = 8;
pub const PopplerAnnotType_POPPLER_ANNOT_HIGHLIGHT: PopplerAnnotType = 9;
pub const PopplerAnnotType_POPPLER_ANNOT_UNDERLINE: PopplerAnnotType = 10;
pub const PopplerAnnotType_POPPLER_ANNOT_SQUIGGLY: PopplerAnnotType = 11;
pub const PopplerAnnotType_POPPLER_ANNOT_STRIKE_OUT: PopplerAnnotType = 12;
pub const PopplerAnnotType_POPPLER_ANNOT_STAMP: PopplerAnnotType = 13;
pub const PopplerAnnotType_POPPLER_ANNOT_CARET: PopplerAnnotType = 14;
pub const PopplerAnnotType_POPPLER_ANNOT_INK: PopplerAnnotType = 15;
pub const PopplerAnnotType_POPPLER_ANNOT_POPUP: PopplerAnnotType = 16;
pub const PopplerAnnotType_POPPLER_ANNOT_FILE_ATTACHMENT: PopplerAnnotType = 17;
pub const PopplerAnnotType_POPPLER_ANNOT_SOUND: PopplerAnnotType = 18;
pub const PopplerAnnotType_POPPLER_ANNOT_MOVIE: PopplerAnnotType = 19;
pub const PopplerAnnotType_POPPLER_ANNOT_WIDGET: PopplerAnnotType = 20;
pub const PopplerAnnotType_POPPLER_ANNOT_SCREEN: PopplerAnnotType = 21;
pub const PopplerAnnotType_POPPLER_ANNOT_PRINTER_MARK: PopplerAnnotType = 22;
pub const PopplerAnnotType_POPPLER_ANNOT_TRAP_NET: PopplerAnnotType = 23;
pub const PopplerAnnotType_POPPLER_ANNOT_WATERMARK: PopplerAnnotType = 24;
pub const PopplerAnnotType_POPPLER_ANNOT_3D: PopplerAnnotType = 25;
pub type PopplerAnnotType = u32;




pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_UNKNOWN: PopplerAnnotFlag = 0;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_INVISIBLE: PopplerAnnotFlag = 1;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_HIDDEN: PopplerAnnotFlag = 2;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_PRINT: PopplerAnnotFlag = 4;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_NO_ZOOM: PopplerAnnotFlag = 8;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_NO_ROTATE: PopplerAnnotFlag = 16;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_NO_VIEW: PopplerAnnotFlag = 32;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_READ_ONLY: PopplerAnnotFlag = 64;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_LOCKED: PopplerAnnotFlag = 128;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_TOGGLE_NO_VIEW: PopplerAnnotFlag = 256;
pub const PopplerAnnotFlag_POPPLER_ANNOT_FLAG_LOCKED_CONTENTS: PopplerAnnotFlag = 512;
pub type PopplerAnnotFlag = u32;



pub const PopplerAnnotMarkupReplyType_POPPLER_ANNOT_MARKUP_REPLY_TYPE_R:
    PopplerAnnotMarkupReplyType = 0;
pub const PopplerAnnotMarkupReplyType_POPPLER_ANNOT_MARKUP_REPLY_TYPE_GROUP:
    PopplerAnnotMarkupReplyType = 1;
pub type PopplerAnnotMarkupReplyType = u32;



pub const PopplerAnnotExternalDataType_POPPLER_ANNOT_EXTERNAL_DATA_MARKUP_3D:
    PopplerAnnotExternalDataType = 0;
pub const PopplerAnnotExternalDataType_POPPLER_ANNOT_EXTERNAL_DATA_MARKUP_UNKNOWN:
    PopplerAnnotExternalDataType = 1;
pub type PopplerAnnotExternalDataType = u32;



pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_MARKED: PopplerAnnotTextState = 0;
pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_UNMARKED: PopplerAnnotTextState = 1;
pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_ACCEPTED: PopplerAnnotTextState = 2;
pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_REJECTED: PopplerAnnotTextState = 3;
pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_CANCELLED: PopplerAnnotTextState = 4;
pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_COMPLETED: PopplerAnnotTextState = 5;
pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_NONE: PopplerAnnotTextState = 6;
pub const PopplerAnnotTextState_POPPLER_ANNOT_TEXT_STATE_UNKNOWN: PopplerAnnotTextState = 7;
pub type PopplerAnnotTextState = u32;



pub const PopplerAnnotFreeTextQuadding_POPPLER_ANNOT_FREE_TEXT_QUADDING_LEFT_JUSTIFIED:
    PopplerAnnotFreeTextQuadding = 0;
pub const PopplerAnnotFreeTextQuadding_POPPLER_ANNOT_FREE_TEXT_QUADDING_CENTERED:
    PopplerAnnotFreeTextQuadding = 1;
pub const PopplerAnnotFreeTextQuadding_POPPLER_ANNOT_FREE_TEXT_QUADDING_RIGHT_JUSTIFIED:
    PopplerAnnotFreeTextQuadding = 2;
pub type PopplerAnnotFreeTextQuadding = u32;



pub const PopplerMoviePlayMode_POPPLER_MOVIE_PLAY_MODE_ONCE: PopplerMoviePlayMode = 0;
pub const PopplerMoviePlayMode_POPPLER_MOVIE_PLAY_MODE_OPEN: PopplerMoviePlayMode = 1;
pub const PopplerMoviePlayMode_POPPLER_MOVIE_PLAY_MODE_REPEAT: PopplerMoviePlayMode = 2;
pub const PopplerMoviePlayMode_POPPLER_MOVIE_PLAY_MODE_PALINDROME: PopplerMoviePlayMode = 3;
/// PopplerMoviePlayMode:
/// @POPPLER_MOVIE_PLAY_MODE_ONCE: the movie should be played once and controls should be closed at the end.
/// @POPPLER_MOVIE_PLAY_MODE_OPEN: the movie should be played once, but controls should be left open.
/// @POPPLER_MOVIE_PLAY_MODE_REPEAT: the movie should be played in loop, until manually stopped.
/// @POPPLER_MOVIE_PLAY_MODE_PALINDROME: the movie should be played forward and backward, forward and backward,
///   and so forth, until manually stopped.
///
/// Play mode enum values.
///
/// Since: 0.54
pub type PopplerMoviePlayMode = u32;



pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_CONTENT:
    PopplerStructureElementKind = 0;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_OBJECT_REFERENCE:
    PopplerStructureElementKind = 1;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_DOCUMENT:
    PopplerStructureElementKind = 2;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_PART: PopplerStructureElementKind =
    3;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_ARTICLE:
    PopplerStructureElementKind = 4;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_SECTION:
    PopplerStructureElementKind = 5;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_DIV: PopplerStructureElementKind =
    6;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_SPAN: PopplerStructureElementKind =
    7;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_QUOTE: PopplerStructureElementKind =
    8;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_NOTE: PopplerStructureElementKind =
    9;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_REFERENCE:
    PopplerStructureElementKind = 10;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_BIBENTRY:
    PopplerStructureElementKind = 11;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_CODE: PopplerStructureElementKind =
    12;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_LINK: PopplerStructureElementKind =
    13;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_ANNOT: PopplerStructureElementKind =
    14;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_BLOCKQUOTE:
    PopplerStructureElementKind = 15;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_CAPTION:
    PopplerStructureElementKind = 16;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_NONSTRUCT:
    PopplerStructureElementKind = 17;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_TOC: PopplerStructureElementKind =
    18;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_TOC_ITEM:
    PopplerStructureElementKind = 19;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_INDEX: PopplerStructureElementKind =
    20;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_PRIVATE:
    PopplerStructureElementKind = 21;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_PARAGRAPH:
    PopplerStructureElementKind = 22;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_HEADING:
    PopplerStructureElementKind = 23;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_HEADING_1:
    PopplerStructureElementKind = 24;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_HEADING_2:
    PopplerStructureElementKind = 25;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_HEADING_3:
    PopplerStructureElementKind = 26;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_HEADING_4:
    PopplerStructureElementKind = 27;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_HEADING_5:
    PopplerStructureElementKind = 28;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_HEADING_6:
    PopplerStructureElementKind = 29;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_LIST: PopplerStructureElementKind =
    30;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_LIST_ITEM:
    PopplerStructureElementKind = 31;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_LIST_LABEL:
    PopplerStructureElementKind = 32;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_LIST_BODY:
    PopplerStructureElementKind = 33;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_TABLE: PopplerStructureElementKind =
    34;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_TABLE_ROW:
    PopplerStructureElementKind = 35;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_TABLE_HEADING:
    PopplerStructureElementKind = 36;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_TABLE_DATA:
    PopplerStructureElementKind = 37;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_TABLE_HEADER:
    PopplerStructureElementKind = 38;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_TABLE_FOOTER:
    PopplerStructureElementKind = 39;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_TABLE_BODY:
    PopplerStructureElementKind = 40;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_RUBY: PopplerStructureElementKind =
    41;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_RUBY_BASE_TEXT:
    PopplerStructureElementKind = 42;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_RUBY_ANNOT_TEXT:
    PopplerStructureElementKind = 43;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_RUBY_PUNCTUATION:
    PopplerStructureElementKind = 44;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_WARICHU:
    PopplerStructureElementKind = 45;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_WARICHU_TEXT:
    PopplerStructureElementKind = 46;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_WARICHU_PUNCTUATION:
    PopplerStructureElementKind = 47;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_FIGURE:
    PopplerStructureElementKind = 48;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_FORMULA:
    PopplerStructureElementKind = 49;
pub const PopplerStructureElementKind_POPPLER_STRUCTURE_ELEMENT_FORM: PopplerStructureElementKind =
    50;
/// PopplerStructureElementKind:
pub type PopplerStructureElementKind = u32;



pub const PopplerStructureGetTextFlags_POPPLER_STRUCTURE_GET_TEXT_NONE:
    PopplerStructureGetTextFlags = 0;
pub const PopplerStructureGetTextFlags_POPPLER_STRUCTURE_GET_TEXT_RECURSIVE:
    PopplerStructureGetTextFlags = 1;
/// PopplerStructureGetTextFlags:
/// @POPPLER_STRUCTURE_GET_TEXT_NONE: No flags.
/// @POPPLER_STRUCTURE_GET_TEXT_RECURSIVE: For non-leaf, non-content
///    elements, recursively obtain the text from all the elements
///    enclosed in the subtree.
pub type PopplerStructureGetTextFlags = u32;



pub const PopplerStructurePlacement_POPPLER_STRUCTURE_PLACEMENT_BLOCK: PopplerStructurePlacement =
    0;
pub const PopplerStructurePlacement_POPPLER_STRUCTURE_PLACEMENT_INLINE: PopplerStructurePlacement =
    1;
pub const PopplerStructurePlacement_POPPLER_STRUCTURE_PLACEMENT_BEFORE: PopplerStructurePlacement =
    2;
pub const PopplerStructurePlacement_POPPLER_STRUCTURE_PLACEMENT_START: PopplerStructurePlacement =
    3;
pub const PopplerStructurePlacement_POPPLER_STRUCTURE_PLACEMENT_END: PopplerStructurePlacement = 4;
/// PopplerStructurePlacement:
pub type PopplerStructurePlacement = u32;



pub const PopplerStructureWritingMode_POPPLER_STRUCTURE_WRITING_MODE_LR_TB:
    PopplerStructureWritingMode = 0;
pub const PopplerStructureWritingMode_POPPLER_STRUCTURE_WRITING_MODE_RL_TB:
    PopplerStructureWritingMode = 1;
pub const PopplerStructureWritingMode_POPPLER_STRUCTURE_WRITING_MODE_TB_RL:
    PopplerStructureWritingMode = 2;
/// PopplerStructureWritingMode:
pub type PopplerStructureWritingMode = u32;



pub const PopplerStructureBorderStyle_POPPLER_STRUCTURE_BORDER_STYLE_NONE:
    PopplerStructureBorderStyle = 0;
pub const PopplerStructureBorderStyle_POPPLER_STRUCTURE_BORDER_STYLE_HIDDEN:
    PopplerStructureBorderStyle = 1;
pub const PopplerStructureBorderStyle_POPPLER_STRUCTURE_BORDER_STYLE_DOTTED:
    PopplerStructureBorderStyle = 2;
pub const PopplerStructureBorderStyle_POPPLER_STRUCTURE_BORDER_STYLE_DASHED:
    PopplerStructureBorderStyle = 3;
pub const PopplerStructureBorderStyle_POPPLER_STRUCTURE_BORDER_STYLE_SOLID:
    PopplerStructureBorderStyle = 4;
pub const PopplerStructureBorderStyle_POPPLER_STRUCTURE_BORDER_STYLE_DOUBLE:
    PopplerStructureBorderStyle = 5;
pub const PopplerStructureBorderStyle_POPPLER_STRUCTURE_BORDER_STYLE_GROOVE:
    PopplerStructureBorderStyle = 6;
pub const PopplerStructureBorderStyle_POPPLER_STRUCTURE_BORDER_STYLE_INSET:
    PopplerStructureBorderStyle = 7;
pub const PopplerStructureBorderStyle_POPPLER_STRUCTURE_BORDER_STYLE_OUTSET:
    PopplerStructureBorderStyle = 8;
/// PopplerStructureBorderStyle:
pub type PopplerStructureBorderStyle = u32;



pub const PopplerStructureTextAlign_POPPLER_STRUCTURE_TEXT_ALIGN_START: PopplerStructureTextAlign =
    0;
pub const PopplerStructureTextAlign_POPPLER_STRUCTURE_TEXT_ALIGN_CENTER: PopplerStructureTextAlign =
    1;
pub const PopplerStructureTextAlign_POPPLER_STRUCTURE_TEXT_ALIGN_END: PopplerStructureTextAlign = 2;
pub const PopplerStructureTextAlign_POPPLER_STRUCTURE_TEXT_ALIGN_JUSTIFY:
    PopplerStructureTextAlign = 3;
/// PopplerStructureTextAlign:
pub type PopplerStructureTextAlign = u32;



pub const PopplerStructureBlockAlign_POPPLER_STRUCTURE_BLOCK_ALIGN_BEFORE:
    PopplerStructureBlockAlign = 0;
pub const PopplerStructureBlockAlign_POPPLER_STRUCTURE_BLOCK_ALIGN_MIDDLE:
    PopplerStructureBlockAlign = 1;
pub const PopplerStructureBlockAlign_POPPLER_STRUCTURE_BLOCK_ALIGN_AFTER:
    PopplerStructureBlockAlign = 2;
pub const PopplerStructureBlockAlign_POPPLER_STRUCTURE_BLOCK_ALIGN_JUSTIFY:
    PopplerStructureBlockAlign = 3;
/// PopplerStructureBlockAlign:
pub type PopplerStructureBlockAlign = u32;



pub const PopplerStructureInlineAlign_POPPLER_STRUCTURE_INLINE_ALIGN_START:
    PopplerStructureInlineAlign = 0;
pub const PopplerStructureInlineAlign_POPPLER_STRUCTURE_INLINE_ALIGN_CENTER:
    PopplerStructureInlineAlign = 1;
pub const PopplerStructureInlineAlign_POPPLER_STRUCTURE_INLINE_ALIGN_END:
    PopplerStructureInlineAlign = 2;
/// PopplerStructureInlineAlign:
pub type PopplerStructureInlineAlign = u32;



pub const PopplerStructureTextDecoration_POPPLER_STRUCTURE_TEXT_DECORATION_NONE:
    PopplerStructureTextDecoration = 0;
pub const PopplerStructureTextDecoration_POPPLER_STRUCTURE_TEXT_DECORATION_UNDERLINE:
    PopplerStructureTextDecoration = 1;
pub const PopplerStructureTextDecoration_POPPLER_STRUCTURE_TEXT_DECORATION_OVERLINE:
    PopplerStructureTextDecoration = 2;
pub const PopplerStructureTextDecoration_POPPLER_STRUCTURE_TEXT_DECORATION_LINETHROUGH:
    PopplerStructureTextDecoration = 3;
/// PopplerStructureTextDecoration:
pub type PopplerStructureTextDecoration = u32;



pub const PopplerStructureRubyAlign_POPPLER_STRUCTURE_RUBY_ALIGN_START: PopplerStructureRubyAlign =
    0;
pub const PopplerStructureRubyAlign_POPPLER_STRUCTURE_RUBY_ALIGN_CENTER: PopplerStructureRubyAlign =
    1;
pub const PopplerStructureRubyAlign_POPPLER_STRUCTURE_RUBY_ALIGN_END: PopplerStructureRubyAlign = 2;
pub const PopplerStructureRubyAlign_POPPLER_STRUCTURE_RUBY_ALIGN_JUSTIFY:
    PopplerStructureRubyAlign = 3;
pub const PopplerStructureRubyAlign_POPPLER_STRUCTURE_RUBY_ALIGN_DISTRIBUTE:
    PopplerStructureRubyAlign = 4;
/// PopplerStructureRubyAlign:
pub type PopplerStructureRubyAlign = u32;



pub const PopplerStructureRubyPosition_POPPLER_STRUCTURE_RUBY_POSITION_BEFORE:
    PopplerStructureRubyPosition = 0;
pub const PopplerStructureRubyPosition_POPPLER_STRUCTURE_RUBY_POSITION_AFTER:
    PopplerStructureRubyPosition = 1;
pub const PopplerStructureRubyPosition_POPPLER_STRUCTURE_RUBY_POSITION_WARICHU:
    PopplerStructureRubyPosition = 2;
pub const PopplerStructureRubyPosition_POPPLER_STRUCTURE_RUBY_POSITION_INLINE:
    PopplerStructureRubyPosition = 3;
/// PopplerStructureRubyPosition:
pub type PopplerStructureRubyPosition = u32;



pub const PopplerStructureGlyphOrientation_POPPLER_STRUCTURE_GLYPH_ORIENTATION_AUTO:
    PopplerStructureGlyphOrientation = 0;
pub const PopplerStructureGlyphOrientation_POPPLER_STRUCTURE_GLYPH_ORIENTATION_0:
    PopplerStructureGlyphOrientation = 0;
pub const PopplerStructureGlyphOrientation_POPPLER_STRUCTURE_GLYPH_ORIENTATION_90:
    PopplerStructureGlyphOrientation = 1;
pub const PopplerStructureGlyphOrientation_POPPLER_STRUCTURE_GLYPH_ORIENTATION_180:
    PopplerStructureGlyphOrientation = 2;
pub const PopplerStructureGlyphOrientation_POPPLER_STRUCTURE_GLYPH_ORIENTATION_270:
    PopplerStructureGlyphOrientation = 3;
/// PopplerStructureGlyphOrientation:
pub type PopplerStructureGlyphOrientation = u32;



pub const PopplerStructureListNumbering_POPPLER_STRUCTURE_LIST_NUMBERING_NONE:
    PopplerStructureListNumbering = 0;
pub const PopplerStructureListNumbering_POPPLER_STRUCTURE_LIST_NUMBERING_DISC:
    PopplerStructureListNumbering = 1;
pub const PopplerStructureListNumbering_POPPLER_STRUCTURE_LIST_NUMBERING_CIRCLE:
    PopplerStructureListNumbering = 2;
pub const PopplerStructureListNumbering_POPPLER_STRUCTURE_LIST_NUMBERING_SQUARE:
    PopplerStructureListNumbering = 3;
pub const PopplerStructureListNumbering_POPPLER_STRUCTURE_LIST_NUMBERING_DECIMAL:
    PopplerStructureListNumbering = 4;
pub const PopplerStructureListNumbering_POPPLER_STRUCTURE_LIST_NUMBERING_UPPER_ROMAN:
    PopplerStructureListNumbering = 5;
pub const PopplerStructureListNumbering_POPPLER_STRUCTURE_LIST_NUMBERING_LOWER_ROMAN:
    PopplerStructureListNumbering = 6;
pub const PopplerStructureListNumbering_POPPLER_STRUCTURE_LIST_NUMBERING_UPPER_ALPHA:
    PopplerStructureListNumbering = 7;
pub const PopplerStructureListNumbering_POPPLER_STRUCTURE_LIST_NUMBERING_LOWER_ALPHA:
    PopplerStructureListNumbering = 8;
/// PopplerStructureListNumbering:
pub type PopplerStructureListNumbering = u32;



pub const PopplerStructureFormRole_POPPLER_STRUCTURE_FORM_ROLE_UNDEFINED: PopplerStructureFormRole =
    0;
pub const PopplerStructureFormRole_POPPLER_STRUCTURE_FORM_ROLE_RADIO_BUTTON:
    PopplerStructureFormRole = 1;
pub const PopplerStructureFormRole_POPPLER_STRUCTURE_FORM_ROLE_PUSH_BUTTON:
    PopplerStructureFormRole = 2;
pub const PopplerStructureFormRole_POPPLER_STRUCTURE_FORM_ROLE_TEXT_VALUE:
    PopplerStructureFormRole = 3;
pub const PopplerStructureFormRole_POPPLER_STRUCTURE_FORM_ROLE_CHECKBOX: PopplerStructureFormRole =
    4;
/// PopplerStructureFormRole:
pub type PopplerStructureFormRole = u32;



pub const PopplerStructureFormState_POPPLER_STRUCTURE_FORM_STATE_ON: PopplerStructureFormState = 0;
pub const PopplerStructureFormState_POPPLER_STRUCTURE_FORM_STATE_OFF: PopplerStructureFormState = 1;
pub const PopplerStructureFormState_POPPLER_STRUCTURE_FORM_STATE_NEUTRAL:
    PopplerStructureFormState = 2;
/// PopplerStructureFormState:
pub type PopplerStructureFormState = u32;



pub const PopplerStructureTableScope_POPPLER_STRUCTURE_TABLE_SCOPE_ROW: PopplerStructureTableScope =
    0;
pub const PopplerStructureTableScope_POPPLER_STRUCTURE_TABLE_SCOPE_COLUMN:
    PopplerStructureTableScope = 1;
pub const PopplerStructureTableScope_POPPLER_STRUCTURE_TABLE_SCOPE_BOTH:
    PopplerStructureTableScope = 2;
/// PopplerStructureTableScope:
pub type PopplerStructureTableScope = u32;


