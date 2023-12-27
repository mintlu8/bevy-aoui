//! Bevy Aoui is a 2D and UI solution for the bevy engine.
//! 
//!
//! # Getting Started
//! 
//! First add the Aoui Plugin:
//! 
//! ```
//! # /*
//! app.add_plugins(AouiPlugin)
//! # */
//! ```
//! 
//! Import the [DSL prelude](dsl::prelude) in the function scope 
//! (it will pollute your namespace otherwise).
//! 
//! 
//! ```
//! # /*
//! fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
//!     use bevy_aoui::dsl::prelude::*;
//!     ...
//! }
//! # */
//! ```
//! 
//! If you don't like the DSL you can try using our [bundles] or [widget builders](crate::dsl::builders).;
//! 
//! Create a sprite:
//! 
//! ```
//! # /*
//! sprite!(commands {
//!     sprite: "Ferris.png",
//!     anchor: Left,
//!     offset: [40, 0],
//!     dimension: [200, 200],
//! })
//! # */
//! ```
//! 
//! This spawns a "Ferris.png" to the center left of the screen,
//! moved to the right by 40 px, with dimension 200 px * 200 px.
//! 
//! Create a stack of words:
//! 
//! ```
//! # /*
//! vbox!(commands {
//!     font_size: em(2),
//!     child: text! {
//!         text: "Hello"
//!     },
//!     child: text! {
//!         text: "rust"
//!     },
//!     child: text! {
//!         text: "and"
//!     },
//!     child: text! {
//!         text: "bevy"
//!     },
//! });
//! # */
//! ```
//! 
//! # What `bevy_aoui` provides:
//! 
//! * Fine grained low level anchor-offset layout system.
//! * First class support for rotation and scaling.
//! * Simple and intuitive containers.
//! * Native ECS components with no external context.
//! * Complete support of bevy's 2D primitives.
//! * Input handling system for mouse and cursor.
//! * Building blocks for most common widgets.
//! * Event handling through one-shot systems.
//! * Reactivity and animation through marked signals.
//! * `macro_rules` based DSL that annihilates boilerplate.
//! * Easy integration with third-party 2D crates.
//! * Easy migration to future bevy versions.
//! 
//! # What' `bevy_aoui` is not
//! 
//! * Not a renderer.
//! 
//!     `bevy_aoui` has minimal rendering features and no third party bevy dependencies,
//!     this ensures maintainability and easy migration to future bevy versions, 
//!     at the cost of not having out of the box widget styles.
//! 
//! * Not `bevy_ui` compatible.
//! 
//!     `bevy_aoui` is not dependent on `bevy_ui` in any way. This means `bevy_ui` exclusive
//!     features won't be available in `bevy_aoui` as is.
//! 
//! * No ui script or serialization.
//!     
//!     `bevy_aoui` uses rust code for a lot of things, including events and reactivity, 
//!     serializing the UI might be difficult.
//! 
//! # The Anchor-Offset Layout System
//!
//! Aoui offers a refreshingly different paradigm from traditional CSS based UI layout.
//! 
//! The core attributes of this system include:
//! * [anchor](Transform2D::anchor)
//! * [center](Transform2D::center)
//! * [offset](Transform2D::offset)
//! * [rotation](Transform2D::rotation)
//! * [scale](Transform2D::scale)
//! * [dimension](Dimension::dim)
//! 
//! Sprites are rectangles, they are connected to parent sprites via one of the parent's anchors `anchor`
//! and translated by `offset`. Then they are scaled and rotated around the sprite's `center`,
//! which can be different from `anchor`.
//! 
//! In the case of parentless sprites, they are anchored to the window's rectangle.
//! 
//! Dimension determines the size of the rectangle.
//! Our relative size system allows dimension to be based on various sources:
//! fixed pixels, sprite/text size, font size, parent size, sprite aspect ratio, etc.
//! 
//! # Container
//! 
//! Anchor-Offset offers fine-grained control over the layout, but you can surrender
//! that control to [containers](layout) for ergonomics.
//! 
//! The `Container` is a very simple layout system that
//! only depends on insertion order of its children. You can find your
//! [`hbox`](layout::CompactLayout), [`grid`](layout::FixedGridLayout) or [`paragraph`](layout::ParagraphLayout) here.
//! 
//! You can implement [`Layout`](layout::Layout) yourself to create a custom layout.
//! 
//! # Widget Abstractions
//! 
//! [widget builders](crate::dsl::builders) are used to empower our DSL.
//! Widget builders implements [`Widget`](dsl::Widget) and [`Default`] and can be used in general like so:
//! 
//! ```
//! # /*
//! FrameBuilder {
//!     offset: [121, 423].dinto(),
//!     anchor: Center.dinto(),
//!     color: color!(red).dinto()
//!     ..Default::default()
//! }.build(commands)
//! # */
//! ```
//! 
//! This returns an [`Entity`](bevy::ecs::entity::Entity).
//! 
//! `dinto` is implemented in [`DslFrom`](dsl::DslFrom) or [`DslInto`](dsl::DslInto). 
//! which gives us nice conversion like `[i32; 2] -> Vec2`, which saves us a lot of typing!
//! 
//! When using the dsl macro, this becomes 
//! ```
//! # /*
//! frame! (commands {
//!     offset: [121, 423],
//!     anchor: Center,
//!     color: color!(red),
//! });
//! # */
//! ```
//! 
//! much nicer, right?
//! 
//! `commands` is the context, if `AssetServer` is needed 
//! we can put `commands` there.
//!
//! # DSL Syntax
//! 
//! The DSL have a few special fields that makes it much more powerful than
//! a simple struct constructor.
//! 
//! ## child and children
//! 
//! `child:` is a special field that can be repeated, it accepts an `Entity`
//! and inserts it as a child.
//! 
//! ```
//! # /*
//! frame! (commands {
//!     ...
//!     child: rectangle! {
//!         dimension: [40, 40]
//!     },
//!     child: text! {
//!         text: "Hello, World!!"
//!     },
//! });
//! # */
//! ```
//! 
//! This syntax, notice the use of braces `{}`,
//! ```
//! field: macro! { .. },
//! ```
//! 
//! Will be automatically rewritten as
//! ```
//! field: macro!(commands { .. }),
//! ```
//! 
//! Which serves as context propagation. 
//! 
//! `children:` adds an iterator as children to the entity.
//! Iterators of `Entity` and `&Entity` are both accepted.
//! Child and children guarantees insertion order.
//! 
//! ## extra
//! 
//! Extra adds a component or a bundle to a widget,
//! which is the idiomatic pattern to compose behaviors.
//! 
//! ```
//! # /*
//! // Example: Add dragging support to a `Sprite`.
//! sprite! (commands {
//!     ...
//!     extra: DragX,
//!     extra: DragConstraint,
//!     extra: DragSnapBack,
//! });
//! # */
//! ```
//! 
//! ## entity
//! 
//! `entity` lets us fetch the [`Entity`](bevy::ecs::entity::Entity)
//! directly from a nested macro invocation.
//! ```
//! # /*
//! let sprite_entity: Entity;
//! sprite! (commands {
//!     child: sprite! {
//!         entity: sprite_entity,
//!     }
//! });
//! # */
//! ```
//! 
//! # Next Steps
//! 
//! Checkout our modules for more documentations and examples.
//! 
//! * [events]
//! * [signals]
//! * [widgets]
//! * [animation](anim)
//! 
//! 
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::single_match)]
pub mod layout;
pub(crate) mod core;
pub mod dsl;
pub mod widgets;
pub mod events;
pub mod anim;

pub mod signals;
pub use core::*;

#[doc(hidden)]
pub use bevy;

pub mod schedule;
mod extension;
pub use extension::WorldExtension;

pub mod util;

pub use schedule::CorePlugin;

/// Plugin for both widgets and events.
#[derive(Debug)]
pub struct AouiPlugin;

impl bevy::prelude::Plugin for AouiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(schedule::CorePlugin)
            .add_plugins(signals::SignalsPlugin)
            .add_plugins(events::CursorEventsPlugin)
            .add_plugins(anim::AnimationPlugin)
            .add_plugins(widgets::WidgetsPlugin)
        ;
    }
}