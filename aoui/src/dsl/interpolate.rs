


/// Construct `Interpolate` components using syntax similar to CSS.
/// 
/// You can create multiple in one macro invocation, separated by semicolon `;`.
/// 
/// 
/// # Syntax
/// 
/// * Default
/// ```
/// # /*
/// transition!(Rotation 1 CubicInOut default PI / 2.0)
/// # */
/// ```
/// 
/// `Interpolate` in default mode waits for systems like signals to set its target.
/// 
/// Since `Interpolate` manages the corresponding field, 
/// the default value here is needed 
/// and will overwrite the corresponded field.
/// 
/// * Repeat/Looping
/// 
/// Repeat and looping will automatically run the animation.
/// 
/// ```
/// # /*
/// transition!(
///     Rotation 3.0 CubicInOut repeat (0.0, PI);
///     Color 3.0 CubicInOut looping [red, blue];
/// )
/// # */
/// ```
/// 
/// Repeat's time value goes from `0->1, 0->1, ...`
/// 
/// Looping's time value goes from `0->1->0->1, ...`
/// 
/// 
/// `Color` automatically uses the `color!` or `gradient!` macro's syntax.
/// 
#[macro_export]
macro_rules! transition {
    ($($tt:tt)*) => {
        $crate::transition_impl!({} $($tt)*)
    };
}

#[macro_export]
macro_rules! easing {
    (Linear) => {$crate::anim::Easing::Linear};
    {$ident: ident} => {$crate::anim::Easing::Ease($crate::anim::EaseFunction::$ident)};
    [$a: expr, $b: expr, $c: expr, $d: expr] => {$crate::anim::Easing::Bezier([
        $a as f32,
        $b as f32,
        $c as f32,
        $d as f32,
    ])};
}
#[doc(hidden)]
#[macro_export]
macro_rules! transition_impl {
    ({$($out: expr),*}) => {($($out),*)};
    ({$($out: expr),*} Color $time:tt $ease:tt default $value:expr $(;$($rest:tt)*)?) => {
        $crate::transition_impl!({   
            $($out,)*
            $crate::anim::Interpolate::<$crate::bevy::prelude::Color>::new(
                $crate::easing!($ease), 
                $crate::colorv4!($value), 
                $time as f32
            )
        }
        $($($rest)*)?)
    };
    ({$($out: expr),*} $name:ident $time:tt $ease:tt default $value:expr $(;$($rest:tt)*)?) => {
        $crate::transition_impl!({   
            $($out,)*
            $crate::anim::Interpolate::<$name>::new(
                $crate::easing!($ease),
                $crate::dsl::DslInto::dinto($value),
                $time as f32
            )
        }
        $($($rest)*)?)
    };

    ({$($out: expr),*} Color $time: tt $ease:tt looping [$($range: tt)*] $(;$($rest:tt)*)?) => {
        $crate::transition_impl!({   
            $($out,)*
            $crate::anim::Interpolate::<$crate::bevy::prelude::Color>::looping(
                $crate::easing!($ease), 
                $crate::gradient!($($range)*),
                $time as f32
            )
        }
        $($($rest)*)?)
    };
    ({$($out: expr),*} $name:ident $time: tt $ease:tt looping $range: expr $(;$($rest:tt)*)?) => {
        $crate::transition_impl!({   
            $($out,)*
            $crate::anim::Interpolate::<$name>::looping(
                $crate::easing!($ease),
                $range,
                $time as f32
            )
        }
        $($($rest)*)?)
    };

    ({$($out: expr),*} Color $time: tt $ease:tt repeat [$($range: tt)*]  $(;$($rest:tt)*)?) => {
        $crate::transition_impl!({   
            $($out,)*
            $crate::anim::Interpolate::<$crate::bevy::prelude::Color>::repeat(
                $crate::easing!($ease), 
                $crate::gradient!($($range)*),
                $time as f32
            )
        }
        $($($rest)*)?)
    };
    ({$($out: expr),*} $name:ident $time: tt $ease:tt repeat $range: expr $(;$($rest:tt)*)?) => {
        $crate::transition_impl!({   
            $($out,)*
            $crate::anim::Interpolate::<$name>::repeat(
                $crate::easing!($ease),
                $range,
                $time as f32
            )
        }
        $($($rest)*)?)
    };
} 