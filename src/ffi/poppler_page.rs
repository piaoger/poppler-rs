use super::*;

/// PopplerPageTransition:
/// @type: the type of transtition
/// @alignment: the dimension in which the transition effect shall occur.
/// Only for #POPPLER_PAGE_TRANSITION_SPLIT and #POPPLER_PAGE_TRANSITION_BLINDS transition types
/// @direction: the direction of motion for the transition effect.
/// Only for #POPPLER_PAGE_TRANSITION_SPLIT, #POPPLER_PAGE_TRANSITION_BOX and #POPPLER_PAGE_TRANSITION_FLY
/// transition types
/// @duration: the duration of the transition effect
/// @angle: the direction in which the specified transition effect shall moves,
/// expressed in degrees counterclockwise starting from a left-to-right direction.
/// Only for #POPPLER_PAGE_TRANSITION_WIPE, #POPPLER_PAGE_TRANSITION_GLITTER, #POPPLER_PAGE_TRANSITION_FLY,
/// #POPPLER_PAGE_TRANSITION_COVER, #POPPLER_PAGE_TRANSITION_UNCOVER and #POPPLER_PAGE_TRANSITION_PUSH
/// transition types
/// @scale: the starting or ending scale at which the changes shall be drawn.
/// Only for #POPPLER_PAGE_TRANSITION_FLY transition type
/// @rectangular: whether the area that will be flown is rectangular and opaque.
/// Only for #POPPLER_PAGE_TRANSITION_FLY transition type
///
/// A #PopplerPageTransition structures describes a visual transition
/// to use when moving between pages during a presentation
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PopplerPageTransition {
    pub type_: consts::PopplerPageTransitionType,
    pub alignment: consts::PopplerPageTransitionAlignment,
    pub direction: consts::PopplerPageTransitionDirection,
    pub duration: gint,
    pub angle: gint,
    pub scale: gdouble,
    pub rectangular: gboolean,
    pub duration_real: gdouble,
}