#[doc(hidden)]
#[macro_export]
macro_rules! inline_context {
    ($commands: tt [$($path: tt)*] [$($fields: tt)*]) => {
        $crate::meta_dsl!($commands [$($path)*] {$($fields)*} {} {} {})
    };

    ($commands: tt [$($path: tt)*] [$($out: tt)*] $field: ident: $macro: ident ! {$($expr: tt)*}) => {
        $crate::inline_context!($commands [$($path)*] [
            $($out)*
            $field: $macro! (
                $commands {
                    $($expr)*
                }
            )
        ])
    };
    ($commands: tt [$($path: tt)*] [$($out: tt)*] $field: ident: $macro: ident ! {$($expr: tt)*} ,$($rest: tt)*) => {
        $crate::inline_context!($commands [$($path)*] [
            $($out)*
            $field: $macro! (
                $commands {
                    $($expr)*
                }
            ),
        ] $($rest)*)
    };
    ($commands: tt [$($path: tt)*] [$($out: tt)*] $field: ident: $head: expr, $($rest: tt)*) => {
        $crate::inline_context!($commands [$($path)*] [$($out)* $field: $head,] $($rest)*)
    };

    ($commands: tt [$($path: tt)*] [$($out: tt)*] $field: ident: $head: expr) => {
        $crate::inline_context!($commands [$($path)*] [$($out)* $field: $head])
    };
}

/// The core macro for our DSL.
#[macro_export]
macro_rules! meta_dsl {

    ($commands: tt [$($path: tt)*] {$($fields: tt)*} ) => {
        $crate::inline_context!($commands [$($path)*] [] $($fields)*)
    };

    ($commands: tt [$($path: tt)*]
        {extra: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras,)* $expr}
            {$($children),*}
        )
    };

    ($commands: tt [$($path: tt)*]
        {child: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2),*}
            {$($extras),*}
            {$($children,)* $expr}
        )
    };

    ($commands: tt [$($path: tt)*]
        {$field: ident: $expr: expr $(,$f: ident: $e: expr)* $(,)?}
        {$($f2: ident: $e2: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
    ) => {
        $crate::meta_dsl!($commands
            [$($path)*]
            {$($f: $e),*}
            {$($f2: $e2,)* $field: $expr}
            {$($extras),*}
            {$($children),*}
        )
    };

    (($commands: expr $(,$e:expr)*) [$($path: tt)*] {$(,)?}
        {$($field: ident: $expr: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
    ) => {
        {
            use $crate::dsl::DslInto;
            let extras = ($($extras),*);
            let children = [$($children),*];
            let entity = $($path)* {
                $($field: ($expr).dinto(),)*
                ..Default::default()
            };
            $commands.spawn_aoui((
                entity,
                extras,
                children,
            ))
        }
    };

    ($commands: ident [$($path: tt)*] {$(,)?}
        {$($field: ident: $expr: expr),*}
        {$($extras: expr),*}
        {$($children: expr),*}
    ) => {
        {
            use $crate::dsl::DslInto;
            let extras = ($($extras),*);
            let children = [$($children),*];
            let entity = $($path)* {
                $($field: ($expr).dinto(),)*
                ..Default::default()
            };
            $commands.spawn_aoui((
                entity,
                extras,
                children,
            ))
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! transform2d {
    ($this: expr) => {
        ::bevy_aoui::Transform2D {
            center: $this.center.unwrap_or(::bevy_aoui::Anchor::Inherit),
            anchor: $this.anchor,
            parent_anchor: $this.parent_anchor.unwrap_or(::bevy_aoui::Anchor::Inherit),
            offset: $this.offset,
            rotation: $this.rotation,
            scale: match $this.scale{
                Some($crate::dsl::prelude::OneOrTwo(vec)) => vec,
                None => ::bevy::math::Vec2::ONE,
            },
            z: $this.z
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! dimension {
    ($this: expr) => {
        match $this.dimension {
            Some(size) => ::bevy_aoui::Dimension::owned(size).with_em($this.font_size),
            None => ::bevy_aoui::Dimension::COPIED.with_em($this.font_size),
        }
    }
}

/// Create a widget extension based on the definition of `Frame`
#[macro_export]
macro_rules! widget_extension {
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident { $($fields: tt)* },
        // Due to macro_rules, this shadows self.
        $this: ident,
        $commands: ident,
        components: ($($input: tt)*)
        $(,spawn: (
            $($children: expr $(=> $comp4: expr)? ),* $(,)?
        ))? $(,)?
    ) => {
        $crate::widget_extension2! {
            $(#[$($parent_attr:tt)*])*
            $vis0 struct $name { $($fields)* },
            // Due to macro_rules, this shadows self.
            $this,
            $commands,
            input: ($($input)*),
            components: (),
            dynamic: (),
            pattern: (),
            spawn: ( $($($children $(=> $comp4)? ),*)? )
        }
    };
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident: Sprite { $($fields: tt)* },
        $this: ident,
        $commands: ident,
        components: (  $($input: tt)* )
        $(,spawn: (
            $($children: expr $(=> $comp4: expr)? ),* $(,)?
        ))? $(,)?
    ) => {
        $crate::widget_extension2! {
            $(#[$($parent_attr:tt)*])*
            $vis0 struct $name { 
                pub sprite: ::bevy::prelude::Handle<bevy::prelude::Image>,
                pub size: Option<::bevy::prelude::Vec2>,
                pub color: Option<::bevy::prelude::Color>,
                pub rect: Option<::bevy::prelude::Rect>,
                pub flip: [bool; 2],
                $($fields)* 
            },
            $this,
            $commands,
            input: (
                ::bevy::prelude::Sprite {
                    custom_size: $this.size,
                    color: $this.color.unwrap_or(bevy::prelude::Color::WHITE),
                    rect: $this.rect,
                    flip_x: $this.flip[0],
                    flip_y: $this.flip[1],
                    ..Default::default()
                },
                $this.sprite,
                ::bevy_aoui::bundles::BuildGlobalBundle::default(),
                $($input)*
            ),
            components: (),
            dynamic: (),
            pattern: (),
            spawn: ( $($($children $(=> $comp4)? ),*)? )
        }
    };
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident: Text { $($fields: tt)* },
        $this: ident,
        $commands: ident,
        components: ($($input: tt)*)
        $(,spawn: (
            $($children: expr $(=> $comp4: expr)? ),* $(,)?
        ))? $(,)?
    ) => {
        $crate::widget_extension2! {
            $(#[$($parent_attr:tt)*])*
            $vis0 struct $name { 
                pub text: String,
                pub font: bevy::prelude::Handle<bevy::prelude::Font>,
                /// Note if not specified this is `UNBOUNDED`.
                pub bounds: Option<bevy::prelude::Vec2>,
                pub color: Option<bevy::prelude::Color>,
                pub wrap: bool,
                pub break_line_on: Option<bevy::text::BreakLineOn>,
                $($fields)* 
            },
            $this,
            $commands,
            input: (
                ::bevy::text::Text {
                    sections: vec![::bevy::text::TextSection::new(
                        $this.text,
                        ::bevy::text::TextStyle {
                            font: $this.font,
                            color: $this.color.unwrap_or(::bevy::prelude::Color::WHITE),
                            ..Default::default()
                        }
                    )],
                    linebreak_behavior: if let Some(b) = $this.break_line_on {
                        b
                    } else if $this.wrap {
                        ::bevy::text::BreakLineOn::WordBoundary
                    } else {
                        ::bevy::text::BreakLineOn::NoWrap
                    },
                    ..Default::default()
                },
                match $this.bounds {
                    Some(size) => ::bevy::text::Text2dBounds { size },
                    None => ::bevy::text::Text2dBounds::UNBOUNDED,
                },
                ::bevy::text::TextLayoutInfo::default(),
                Into::<bevy::sprite::Anchor>::into($this.anchor),
                ::bevy_aoui::bundles::BuildGlobalBundle::default()
                $($input)*
            ),
            components: (),
            dynamic: (),
            pattern: (),
            spawn: ( $($($children $(=> $comp4)? ),*)? )
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! widget_extension2 {
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident {
            $(
                $(#[$($attr:tt)*])*
                $vis: vis $field: ident: $ty: ty
            ),* $(,)?
        },
        $this: ident,
        $commands: ident,
        input: (),
        components: ( $($comp: expr),* ),
        dynamic: ($($if: expr => $comp2: expr),*),
        pattern: ($($pat: pat = $pat_field: expr => $comp3: expr),*),
        spawn: (
            $($children: expr $(=> $comp4: expr)? ),*
        )
    ) => {
        #[derive(Debug, Default)]
        $(#[$($parent_attr)*])*
        $vis0 struct $name {
            pub anchor: ::bevy_aoui::Anchor,
            pub parent_anchor: Option<::bevy_aoui::Anchor>,
            pub center: Option<::bevy_aoui::Anchor>,
            /// Propagated opacity
            pub opacity: ::bevy_aoui::Opacity,
            pub visible: Option<bool>,
            pub offset: ::bevy_aoui::Size2,
            pub rotation: f32,
            pub scale: Option<$crate::dsl::OneOrTwo<::bevy::math::Vec2>>,
            pub z: f32,
            pub dimension: Option<::bevy_aoui::Size2>,
            pub font_size: ::bevy_aoui::SetEM,
            pub hitbox: Option<::bevy_aoui::Hitbox>,
            $($(#[$($attr)*])* $vis $field: $ty),*
        }

        const _: () = {
            use $crate::dsl::DslInto;
            use ::bevy::prelude::BuildChildren;
            impl $crate::dsl::AoUIWidget for $name {
                fn spawn_with(self, $commands: &mut ::bevy::prelude::Commands) -> ::bevy::prelude::Entity {
                    let $this = self;
                    let mut base = $commands.spawn((
                        bevy_aoui::bundles::AoUIBundle {
                            transform: $crate::transform2d!($this),
                            dimension: $crate::dimension!($this),
                            opacity: $this.opacity,
                            vis: $this.visible.dinto(),
                            ..Default::default()
                        },
                        $($comp),*
                    ));
                    if let Some(hitbox) = $this.hitbox {
                        base.insert(hitbox);
                    }
                    $(if $if {
                        base.insert($comp2);
                    })*
                    $(if let $pat = $pat_field {
                        base.insert($comp3);
                    })*
                    let base = base.id();
                    let children = [$(
                        {
                            let child = $children;
                            $commands.entity(child)$(.insert($comp4))?.id()
                        }
                    ),*];
                    $commands.entity(base).push_children(&children);
                    base
                }
            }
        };
    };
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident { $($fields: tt)* },
        $this: ident,
        $commands: ident,
        input: ($bundle: expr, $($rest: tt)*),
        components: ( $($comp: expr),* ),
        dynamic: ($($if: expr => $comp2: expr),*),
        pattern: ($($pat: pat = $pat_field: expr => $comp3: expr),*),
        spawn: ( $($children: expr $(=> $comp4: expr)? ),* )
    ) => {
        $crate::widget_extension2! (
            $(#[$($parent_attr)*])*
            $vis0 struct $name { $($fields)* },
            $this,
            $commands,
            input: ($($rest)*),
            components: ( $($comp,)* $bundle),
            dynamic: ($($if => $comp2),*),
            pattern: ($($pat = $pat_field => $comp3),*),
            spawn: ( $($children $(=> $comp4)? ),* )
        );
    };
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident { $($fields: tt)* },
        $this: ident,
        $commands: ident,
        input: ($bundle: expr),
        components: ( $($comp: expr),* ),
        dynamic: ($($if: expr => $comp2: expr),*),
        pattern: ($($pat: pat = $pat_field: expr => $comp3: expr),*),
        spawn: ( $($children: expr $(=> $comp4: expr)? ),* )
    ) => {
        $crate::widget_extension2! (
            $(#[$($parent_attr)*])*
            $vis0 struct $name { $($fields)* },
            $this,
            $commands,
            input: (),
            components: ( $($comp,)* $bundle),
            dynamic: ($($if => $comp2),*),
            pattern: ($($pat = $pat_field => $comp3),*),
            spawn: ( $($children $(=> $comp4)? ),* )
        );
    };
    (
        $(#[$($parent_attr:tt)*])*        
        $vis0: vis struct $name: ident { $($fields: tt)* },
        $this: ident,
        $commands: ident,
        input: ($pat0: pat = $pat_field0: expr => $expr0: expr, $($rest: tt)*),
        components: ( $($comp: expr),* ),
        dynamic: ($($if: expr => $comp2: expr),*),
        pattern: ($($pat: pat = $pat_field: expr => $comp3: expr),*),
        spawn: (
            $($children: expr $(=> $comp4: expr)? ),*
        )
    ) => {
        $crate::widget_extension2! {
            $(#[$($parent_attr)*])*
            $vis0 struct $name { $($fields)* },
            $this,
            $commands,
            input: ($($rest)*),
            components: ($($comp),*),
            dynamic: ($($if => $comp2),*),
            pattern: ($($pat = $pat_field => $comp3,)* $pat0 = $pat_field0 => $expr0),
            spawn: (
                $($children $(=> $comp4)? ),*
            )
        }
    };
    (
        $(#[$($parent_attr:tt)*])*        
        $vis0: vis struct $name: ident { $($fields: tt)* },
        $this: ident,
        $commands: ident,
        input: ($pat0: pat = $pat_field0: expr => $expr0: expr),
        components: ( $($comp: expr),* ),
        dynamic: ($($if: expr => $comp2: expr),*),
        pattern: ($($pat: pat = $pat_field: expr => $comp3: expr),*),
        spawn: (
            $($children: expr $(=> $comp4: expr)? ),*
        )
    ) => {
        $crate::widget_extension2! {
            $(#[$($parent_attr)*])*
            $vis0 struct $name { $($fields)* },
            $this,
            $commands,
            input: (),
            components: ($($comp),*),
            dynamic: ($($if => $comp2),*),
            pattern: ($($pat = $pat_field => $comp3,)* $pat0 = $pat_field0 => $expr0),
            spawn: (
                $($children $(=> $comp4)? ),*
            )
        }
    };
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident { $($fields: tt)* },
        $this: ident,
        $commands: ident,
        input: ($if0: expr => $expr0: expr, $($rest: tt)*),
        components: ( $($comp: expr),* ),
        dynamic: ($($if: expr => $comp2: expr),*),
        pattern: ($($pat: pat = $pat_field: expr => $comp3: expr),*),
        spawn: (
            $($children: expr $(=> $comp4: expr)? ),*
        )
    ) => {
        $crate::widget_extension2! (
            $(#[$($parent_attr)*])*            
            $vis0 struct $name { $($fields)* },
            $this,
            $commands,
            input: ($($rest)*),
            components: ($($comp),*),
            dynamic: ($($if => $comp2,)* $if0 => $expr0),
            pattern: ($($pat = $pat_field => $comp3),*),
            $(,spawn: (
                $($children $(=> $comp4)? ),*
            ))?
        )
    };
    (
        $(#[$($parent_attr:tt)*])*
        $vis0: vis struct $name: ident { $($fields: tt)* },
        $this: ident,
        $commands: ident,
        input: ($if0: expr => $expr0: expr),
        components: ( $($comp: expr),* ),
        dynamic: ($($if: expr => $comp2: expr),*),
        pattern: ($($pat: pat = $pat_field: expr => $comp3: expr),*),
        spawn: (
            $($children: expr $(=> $comp4: expr)? ),*
        )
    ) => {
        $crate::widget_extension2! (
            $(#[$($parent_attr)*])*            
            $vis0 struct $name { $($fields)* },
            $this,
            $commands,
            input: (),
            components: ($($comp),*),
            dynamic: ($($if => $comp2,)* $if0 => $expr0),
            pattern: ($($pat = $pat_field => $comp3),*),
            $(,spawn: (
                $($children $(=> $comp4)? ),*
            ))?
        )
    };
}
