//! UI, events and dsl for the `bevy_aoui` crate.
//! 
//! This crate does not have a stable API and subject to change.
//! 
//! # Event System
//! 
//! Since AoUI presumably sits on top of the bevy app, we provide an event system
//! for detecting cursor activity exclusively for AoUI widgets. Cursor events not
//! catched by our system can be handled by other systems.
//! 
//! We offer a component insertion based core event system for library developers 
//! as well as a oneshot system based event handler system for end users.
//! 
//! # Widgets
//! 
//! We currently offer a few simple widgets. 
//! 
//! * `Shape`: a vector shape renderer using `bevy_prototype_lyon`.
//! * `InputBox`: a single line text input.
//! * `Button`: a widget that provides click detection and propagation.
//! 
//! # DSL
//! 
//! We offer a DSL for streamlining widget construction.
//! 
//! Before you start, always import the prelude for syntax consistancy.
//! ```
//! use bevy_aoui_widgets::dsl::prelude::*;
//! ```
//! 
//! Each "widget" has a struct and its corresponding macro.
//! 
//! ```
//! # /*
//! sprite! ((commands, ..) {
//!     dim: [400, 400],
//!     sprite: assets.load("Ferris.png"),
//! });
//! # */
//! ```
//! This translates to
//! 
//! ```
//! # /*
//! Sprite {
//!     dim: [400, 400].dinto(),
//!     sprite: assets.load("Ferris.png").dinto(),
//!     ..Default::default(),
//! }.spawn_with(&mut commands);
//! # */
//! ```
//! 
//! Where `dinto` is our own [`Into`], [`DslInto`](crate::dsl::DslInto),
//! where all the syntax magic happens.
//! 
//! Add children like so, notice you don't need to manually pass in 
//! the context `(commands, ..)`
//! 
//! ```
//! # /*
//! sprite! ((commands, ..) {
//!     dim: [400, 400],
//!     sprite: assets.load("Ferris.png"),
//!     child: textbox! {
//!         ...
//!     }
//! });
//! # */
//! ```
//! 
//! Check our our book or examples for more info.
pub mod dsl;
pub mod widgets;
pub mod events;
pub mod anim;

mod dto;

pub use dto::Submit;

#[doc(hidden)]
pub use generic_static::StaticTypeMap;

/// Plugin for both widgets and events.
pub struct AoUIExtensionsPlugin;

impl bevy::prelude::Plugin for AoUIExtensionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(events::AoUICursorEventsPlugin)
            .add_plugins(anim::AoUIAnimationPlugin)
            .add_plugins(widgets::schedule::FullWidgetsPlugin)
        ;
    }
}