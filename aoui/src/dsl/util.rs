use bevy::math::Vec2;
use crate::{Hitbox, HitboxShape, Anchor};
use crate::{Size2, SetEM, layout::Alignment, layout::FlexDir};

use crate::util::{Sender, Receiver};

use super::convert::DslInto;

/// Syntax for constructing a hitbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DslHitbox<T: DslInto<OneOrTwo<Vec2>>> {
    Rect(T),
    Ellipse(T),
}

impl<T: DslInto<OneOrTwo<Vec2>>> DslInto<Option<Hitbox>> for DslHitbox<T> {
    fn dinto(self) -> Option<Hitbox> {
        Some(match self {
            DslHitbox::Rect(scale) => Hitbox {
                shape: HitboxShape::Rect,
                scale: scale.dinto().0,
            },
            DslHitbox::Ellipse(scale) =>  Hitbox {
                shape: HitboxShape::Rect,
                scale: scale.dinto().0,
            },
        })
    }
}

/// Unified constants for various enums used by `AoUI`.
/// 
/// Note `Left` can be used as `CenterLeft`, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AoUISpacialConsts {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

impl DslInto<Anchor> for AoUISpacialConsts {
    fn dinto(self) -> Anchor {
        match self {
            AoUISpacialConsts::TopLeft => Anchor::TopLeft,
            AoUISpacialConsts::TopCenter => Anchor::TopCenter,
            AoUISpacialConsts::TopRight => Anchor::TopRight,
            AoUISpacialConsts::CenterLeft => Anchor::CenterLeft,
            AoUISpacialConsts::Center => Anchor::Center,
            AoUISpacialConsts::CenterRight => Anchor::CenterRight,
            AoUISpacialConsts::BottomLeft => Anchor::BottomLeft,
            AoUISpacialConsts::BottomCenter => Anchor::BottomCenter,
            AoUISpacialConsts::BottomRight => Anchor::BottomRight,
            AoUISpacialConsts::Top => Anchor::TopCenter,
            AoUISpacialConsts::Bottom => Anchor::BottomCenter,
            AoUISpacialConsts::Left => Anchor::CenterLeft,
            AoUISpacialConsts::Right => Anchor::CenterRight,
            c => panic!("{:?} is not an Anchor.", c),
        }
    }
}

type BevyAnchor = bevy::sprite::Anchor;

impl DslInto<BevyAnchor> for AoUISpacialConsts {
    fn dinto(self) -> BevyAnchor {
        match self {
            AoUISpacialConsts::TopLeft => BevyAnchor::TopLeft,
            AoUISpacialConsts::TopCenter => BevyAnchor::TopCenter,
            AoUISpacialConsts::TopRight => BevyAnchor::TopRight,
            AoUISpacialConsts::CenterLeft => BevyAnchor::CenterLeft,
            AoUISpacialConsts::Center => BevyAnchor::Center,
            AoUISpacialConsts::CenterRight => BevyAnchor::CenterRight,
            AoUISpacialConsts::BottomLeft => BevyAnchor::BottomLeft,
            AoUISpacialConsts::BottomCenter => BevyAnchor::BottomCenter,
            AoUISpacialConsts::BottomRight => BevyAnchor::BottomRight,
            AoUISpacialConsts::Top => BevyAnchor::TopCenter,
            AoUISpacialConsts::Bottom => BevyAnchor::BottomCenter,
            AoUISpacialConsts::Left => BevyAnchor::CenterLeft,
            AoUISpacialConsts::Right => BevyAnchor::CenterRight,
            c => panic!("{:?} is not an Anchor.", c),
        }
    }
}

impl DslInto<Alignment> for AoUISpacialConsts {
    fn dinto(self) -> Alignment {
        match self {
            AoUISpacialConsts::Center => Alignment::Center,
            AoUISpacialConsts::Top => Alignment::Top,
            AoUISpacialConsts::Bottom => Alignment::Bottom,
            AoUISpacialConsts::Left => Alignment::Left,
            AoUISpacialConsts::Right => Alignment::Right,
            c => panic!("{:?} is not an Alignment.", c),
        }
    }
}

impl DslInto<Option<Alignment>> for AoUISpacialConsts {
    fn dinto(self) -> Option<Alignment> {
        Some(match self {
            AoUISpacialConsts::Center => Alignment::Center,
            AoUISpacialConsts::Top => Alignment::Top,
            AoUISpacialConsts::Bottom => Alignment::Bottom,
            AoUISpacialConsts::Left => Alignment::Left,
            AoUISpacialConsts::Right => Alignment::Right,
            c => panic!("{:?} is not an Alignment.", c),
        })
    }
}

impl DslInto<FlexDir> for AoUISpacialConsts {
    fn dinto(self) -> FlexDir {
        match self {
            AoUISpacialConsts::LeftToRight => FlexDir::LeftToRight,
            AoUISpacialConsts::RightToLeft => FlexDir::RightToLeft,
            AoUISpacialConsts::TopToBottom => FlexDir::TopToBottom,
            AoUISpacialConsts::BottomToTop => FlexDir::BottomToTop,
            c => panic!("{:?} is not an FlexDir.", c),
        }
    }
}

impl DslInto<Option<FlexDir>> for AoUISpacialConsts {
    fn dinto(self) -> Option<FlexDir> {
        Some(match self {
            AoUISpacialConsts::LeftToRight => FlexDir::LeftToRight,
            AoUISpacialConsts::RightToLeft => FlexDir::RightToLeft,
            AoUISpacialConsts::TopToBottom => FlexDir::TopToBottom,
            AoUISpacialConsts::BottomToTop => FlexDir::BottomToTop,
            c => panic!("{:?} is not an FlexDir.", c),
        })
    }
}

/// Color construction macro, see [`colorthis`].
/// 
/// Input is `RgbaLinear`, but immediately cast into `Rgba`(sRGB).
/// 
/// ```
/// # use bevy_aoui::color;
/// color!(red400);
/// ```
#[macro_export]
macro_rules! color {
    ($color: tt) => {
        $crate::dsl::rgbaf!(
            $crate::bevy::prelude::Color::RgbaLinear, 
            $color => {red, green, blue, alpha}
        ).as_rgba()
    };
}

/// Color construction macro, see [`colorthis`]. This constructs a vector4.
#[macro_export]
macro_rules! colorv4 {
    ($color: tt) => {
        $crate::dsl::rgbaf!(
            $crate::bevy::prelude::Color::RgbaLinear, 
            $color => {red, green, blue, alpha}
        ).as_rgba().into()
    };
}

/// Color construction macro, see [`colorthis`]. This constructs a vector4.
#[macro_export]
macro_rules! gradient {
    [$(($color: tt, $frac: expr)),* $(,)?] => {
        [$(($crate::colorv4!($color), $frac)),*]
    };
}


/// Convert degrees to radians
pub fn degrees(f: impl DslInto<f32>) -> f32{
    f32::to_radians(f.dinto())
}

/// Convert `Vec2` to radians
pub fn angle(f: impl DslInto<Vec2>) -> f32{
    let v = f.dinto();
    f32::atan2(v.y, v.x)
}

impl DslInto<SetEM> for i32 {
    fn dinto(self) -> SetEM {
        SetEM::Pixels(self as f32)
    }
}

impl DslInto<SetEM> for f32 {
    fn dinto(self) -> SetEM {
        SetEM::Pixels(self)
    }
}

/// Set font size by `px`.
pub fn px(f: impl DslInto<f32>) -> SetEM {
    SetEM::Pixels(f.dinto())
}

/// Set font size by `em`.
pub fn em(f: impl DslInto<f32>) -> SetEM {
    SetEM::Ems(f.dinto())
}

/// Set font size by `rem`.
pub fn rem(f: impl DslInto<f32>) -> SetEM {
    SetEM::Rems(f.dinto())
}

/// Set font size by `%`.
/// 
/// Provide values like `40`, not `0.4`.
pub fn percent(f: impl DslInto<f32>) -> SetEM {
    SetEM::Pixels(f.dinto() / 100.0)
}

/// Accepts 1 or 2 numbers for a `Vec2` or a `Size2`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OneOrTwo<T>(pub T);

impl<T> DslInto<OneOrTwo<T>> for T {
    fn dinto(self) -> OneOrTwo<T> {
        OneOrTwo(self)
    }
}

impl<T> DslInto<OneOrTwo<[T; 2]>> for T where T : Clone {
    fn dinto(self) -> OneOrTwo<[T; 2]> {
        OneOrTwo([self.clone(), self])
    }
}

macro_rules! impl_one_or_two {
    ($ty: ty, $x: ident, $y: ident, $expr: expr) => {
        
impl DslInto<OneOrTwo<$ty>> for i32 {
    fn dinto(self) -> OneOrTwo<$ty> {
        let $x = self;
        let $y = self;
        OneOrTwo($expr)
    }
}

impl DslInto<OneOrTwo<$ty>> for f32 {
    fn dinto(self) -> OneOrTwo<$ty> {
        let $x = self;
        let $y = self;
        OneOrTwo($expr)
    }
}

impl DslInto<OneOrTwo<$ty>> for [i32; 2] {
    fn dinto(self) -> OneOrTwo<$ty> {
        let [$x, $y] = self;
        OneOrTwo($expr)
    }
}

impl DslInto<OneOrTwo<$ty>> for [f32; 2] {
    fn dinto(self) -> OneOrTwo<$ty> {
        let [$x, $y] = self;
        OneOrTwo($expr)
    }
}

impl DslInto<OneOrTwo<$ty>> for (i32, i32) {
    fn dinto(self) -> OneOrTwo<$ty> {
        let ($x, $y) = self;
        OneOrTwo($expr)
    }
}

impl DslInto<OneOrTwo<$ty>> for (f32, i32) {
    fn dinto(self) -> OneOrTwo<$ty> {
        let ($x, $y) = self;
        OneOrTwo($expr)
    }
}

impl DslInto<OneOrTwo<$ty>> for (i32, f32) {
    fn dinto(self) -> OneOrTwo<$ty> {
        let ($x, $y) = self;
        OneOrTwo($expr)
    }
}

impl DslInto<OneOrTwo<$ty>> for (f32, f32) {
    fn dinto(self) -> OneOrTwo<$ty> {
        let ($x, $y) = self;
        OneOrTwo($expr)
    }
}
    };
}

impl_one_or_two!(Vec2, x, y, Vec2::new(x as f32, y as f32));
impl_one_or_two!(Size2, x, y, Size2::pixels(x as f32, y as f32));

#[doc(hidden)]
#[macro_export]
macro_rules! size {
    ($x: tt) => {
        ($crate::SizeUnit::Pixels, $x as f32)
    };
    (-$x: tt) => {
        ($crate::SizeUnit::Pixels, -($x as f32))
    };
    ($x: tt px) => {
        ($crate::SizeUnit::Pixels, $x as f32)
    };
    (-$x: tt px) => {
        ($crate::SizeUnit::Pixels, -($x as f32))
    };
    ($x: tt em) => {
        ($crate::SizeUnit::Em, $x as f32)
    };
    (-$x: tt em) => {
        ($crate::SizeUnit::Em, -($x as f32))
    };
    ($x: tt rem) => {
        ($crate::SizeUnit::Rem, $x as f32)
    };
    (-$x: tt rem) => {
        ($crate::SizeUnit::Rem, -($x as f32))
    };
    ($x: tt %) => {
        ($crate::SizeUnit::Percent, $x as f32 / 100.0)
    };
    (-$x: tt %) => {
        ($crate::SizeUnit::Percent, -($x as f32) / 100.0)
    };
    (1 - $x: tt px) => {
        ($crate::SizeUnit::MarginPx, $x as f32)
    };
    (1 + $x: tt px) => {
        ($crate::SizeUnit::MarginPx, -($x as f32))
    };
    (1 - $x: tt em) => {
        ($crate::SizeUnit::MarginEm, $x as f32)
    };
    (1 + $x: tt em) => {
        ($crate::SizeUnit::MarginEm, -($x as f32))
    };
    (1 - $x: tt rem) => {
        ($crate::SizeUnit::MarginRem, $x as f32)
    };
    (1 + $x: tt rem) => {
        ($crate::SizeUnit::MarginRem, -($x as f32))
    };
}


/// Construct a [`Size2`](bevy_aoui::Size2) through CSS like syntax.
/// 
/// # Examples
/// ```
/// # use bevy_aoui::size2;
/// # let PI = 3.0;
/// // We perform auto float conversion.
/// size2!([40, 50.5]);
/// // Supply a unit like this
/// size2!([1, 1] rem);
/// // Supply multiple unit types like this.
/// size2!([40%, 1 em]);
/// // Aside from the negative sign
/// // expressions need to be in wrapped parenthesis or braces.
/// size2!([-3 px, (PI * 2.0) rem]);
/// size2!([-3 px, {
///     let pi = 3.0;
///     pi * 2.0
/// } rem]);
/// // `1 - 2px` means `100% - 2px`, or 2px smaller than parent dimension.
/// size2!([1 - 2 px, 1 + 4 em]);
/// // or expressed as
/// size2!(1 - [4.5, 6.6] px);
/// ```
/// 
/// # Note
/// 
/// * `1px` is not valid rust syntax, always use `1 px`.
#[macro_export]
macro_rules! size2 {
    (full) => {
        $crate::Size2::FULL
    };
    ([$($tt:tt)*]) => {
        $crate::size2!(@accumulate [] [$($tt)*])
    };
    (@accumulate [$($tt1:tt)*] []) => {
        compile_error!("Expected 2 expressions, found 1.")
    };
    (@accumulate [$($tt1:tt)*] [, $($tt2:tt)*]) => {
        $crate::Size2::new($crate::size!($($tt1)*), $crate::size!($($tt2)*))
    };
    (@accumulate [$($tt1:tt)*] [$tt:tt $($tt2:tt)*]) => {
        $crate::size2!(@accumulate [$($tt1)* $tt] [$($tt2)*])
    };
    ([$x: expr, $y: expr] $unit: tt)=> {
        $crate::size2!([$x $unit, $y $unit])
    };
    (1 - [$x: expr, $y: expr] $unit: tt)=> {
        $crate::size2!([1 - $x $unit, 1 - $y $unit])
    };
    (1 + [$x: expr, $y: expr] $unit: tt)=> {
        $crate::size2!([1 + $x $unit, 1 + $y $unit])
    };
}


/// This bypasses the option impl on dinto.
#[doc(hidden)]
#[derive(Debug, Default)]
pub enum OptionX<T> {
    Some(T),
    #[default]
    None,
}

impl<T> DslInto<OptionX<Sender<T>>> for Sender<()>{
    fn dinto(self) -> OptionX<Sender<T>> {
        OptionX::Some(self.mark::<T>())
    }
}

impl<T> DslInto<OptionX<Receiver<T>>> for Receiver<()>{
    fn dinto(self) -> OptionX<Receiver<T>> {
        OptionX::Some(self.mark::<T>())
    }
}
