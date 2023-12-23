#![allow(non_upper_case_globals)]

use crate::Anchor;
use crate::BuildTransform;
pub use crate::{color, colors, gradient, transition, size2, markers};
pub use super::DslInto;
pub use super::util::*;
pub use super::util::DslHitbox::*;
pub use super::util::AouiSpacialConsts::*;
pub use super::AouiCommands;
pub use bevy::prelude::BuildChildren;
pub use std::f32::consts::PI;
pub const INFINITY: f32 = f32::INFINITY;
pub const EPS: f32 = f32::EPSILON;
pub use bevy::prelude::Color;
pub use crate::{Dimension, Opacity, SizeUnit, Size2};
pub use crate::layout::LayoutControl::{Linebreak, IgnoreLayout};
pub use crate::anim::{Interpolate, Offset, Rotation, Scale, Index};
pub use interpolation::EaseFunction;
pub use crate::events::{
    EventFlags,
    LeftDown, LeftClick, 
    MidDown, MidClick, 
    RightDown, RightClick,
    DragEnd, Drop, ClickOutside,
    Hover, 
    LeftPressed, LeftDrag,
    MidPressed, MidDrag,
    RightPressed, RightDrag,
    EvLoseFocus,
    CustomCursor,
};
pub use crate::OpacityWriter;
pub use crate::signals::{signal, fps_signal, Sender, Receiver, types::*};
pub use bevy::window::CursorIcon;
pub use crate::widgets::button::{
    PropagateFocus, DisplayIf, SetCursor, CheckButtonState, radio_button_group
};
pub use crate::widgets::scroll::{
    Scrolling, ScrollDirection
};

pub const FlipX: [bool; 2] = [true, false];
pub const FlipY: [bool; 2] = [false, true];
pub const FlipBoth: [bool; 2] = [true, true];

pub const DragX: crate::widgets::drag::Draggable = crate::widgets::drag::Draggable::X;
pub const DragY: crate::widgets::drag::Draggable = crate::widgets::drag::Draggable::Y;
pub const DragBoth: crate::widgets::drag::Draggable = crate::widgets::drag::Draggable::BOTH;
pub const DragSnapBack: crate::widgets::drag::DragSnapBack = crate::widgets::drag::DragSnapBack::DEFAULT;

pub const Inherit: Anchor = Anchor::Inherit;

pub use super::atlas::AtlasRectangles::Grid as AtlasGrid;
pub use super::Aspect::Preserve;

pub use crate::{frame, sprite, text, atlas};
pub use crate::{material_sprite, material_mesh};
pub use crate::{one_shot, handler};
pub use crate::{padding, compact, paragraph, span, hbox, vbox, hspan, vspan};
pub use crate::{linebreak, table, flex_table, fixed_grid, sized_grid,};
pub use crate::{inputbox, button, check_button, radio_button, clipping_layer};
pub use crate::rectangle;

pub use crate::dsl::context::{with_layer, use_opacity, with_marker};

use bevy::ecs::bundle::Bundle;
use bevy::transform::components::GlobalTransform;

/// Build transform at an anchor.
pub fn transform_at(anc: Anchor) -> impl Bundle {
    (
        BuildTransform(anc),
        GlobalTransform::default()
    )
}