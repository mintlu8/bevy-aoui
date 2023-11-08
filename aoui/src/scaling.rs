use bevy::{prelude::{Vec2, Resource}, reflect::Reflect};

/// The root relative size of the window.
/// 
/// By default this is `[16, 16]` if not found.
#[derive(Debug, Resource)]
pub struct AouiREM(pub f32);
impl Default for AouiREM {
    fn default() -> Self {
        Self(16.0)
    }
}

/// Set the em relative to parent.
#[derive(Debug, Clone, Copy, Default, Reflect)]
pub enum SetEM {
    #[default]
    None,
    Pixels(f32),
    Ems(f32),
    Rems(f32),
}

/// The unit of a Size `px`, `em`, `rem`, `percent`
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub enum SizeUnit{
    #[default]
    Pixels,
    Em,
    Rem,
    Percent,
}

/// A context sensitive Vec2
#[derive(Debug, Clone, Copy, PartialEq, Default, Reflect)]
pub struct Size2 {
    x: SizeUnit,
    y: SizeUnit,
    raw: Vec2,
}


impl SizeUnit {

    /// Compute size in pixels given parent info.
    #[inline]
    pub fn as_pixels(self, value: f32, parent: f32, em: f32, rem: f32) -> f32 {
        match self {
            SizeUnit::Pixels => value,
            SizeUnit::Em => value * em,
            SizeUnit::Rem => value * rem,
            SizeUnit::Percent => value * parent,
        }
    }
}

impl Size2 {
    pub const ZERO: Self = Self {
        x: SizeUnit::Pixels,
        y: SizeUnit::Pixels,
        raw: Vec2::ZERO,
    };


    pub const INHERIT: Self = Self {
        x: SizeUnit::Percent,
        y: SizeUnit::Percent,
        raw: Vec2::ONE,
    };

    /// Construct size.
    pub const fn new(x: (SizeUnit, f32), y: (SizeUnit, f32)) -> Self{
        Self {
            x: x.0,
            y: y.0,
            raw: Vec2::new(x.1, y.1)
        }
    }

    /// Size based on fixed number of pixels.
    pub const fn pixels(x: f32, y: f32) -> Self{
        Self {
            x: SizeUnit::Pixels,
            y: SizeUnit::Pixels,
            raw: Vec2::new(x, y),
        }
    }

    /// Size based on the parent relative size.
    pub const fn em(x: f32, y: f32) -> Self{
        Self {
            x: SizeUnit::Em,
            y: SizeUnit::Em,
            raw: Vec2::new(x, y),
        }
    }

    /// Size based on the root size.
    pub const fn rem(x: f32, y: f32) -> Self{
        Self {
            x: SizeUnit::Rem,
            y: SizeUnit::Rem,
            raw: Vec2::new(x, y),
        }
    }

    /// Size based on a percentage for the parent size.
    pub const fn percent(x: f32, y: f32) -> Self{
        Self {
            x: SizeUnit::Percent,
            y: SizeUnit::Percent,
            raw: Vec2::new(x, y),
        }
    }

    /// Compute size in pixels given parent info.
    #[inline]
    pub fn as_pixels(&self, parent: Vec2, em: f32, rem: f32) -> Vec2 {
        Vec2::new(
            self.x.as_pixels(self.raw.x, parent.x, em, rem),
            self.y.as_pixels(self.raw.y, parent.y, em, rem),
        )
    }

    /// Units of x and y.
    pub fn units(&self) -> (SizeUnit, SizeUnit) {
        (self.x, self.y)
    }

    /// A loose function that obtains a vec2 from this struct.
    /// 
    /// The unit and meaning of this value depends on the use case.
    pub fn raw(&self) -> Vec2 {
        self.raw
    }

    /// Get mutable access to the underlying owned value.
    /// 
    /// For ease of use with egui.
    #[doc(hidden)]
    pub fn raw_mut(&mut self) -> &mut Vec2 {
        &mut self.raw
    }

    /// A loose function that updates this struct's value.
    /// 
    /// The unit and meaning of this value depends on the use case.
    pub fn set_raw(&mut self, value: Vec2) {
        self.raw = value
    }

    /// A loose function that updates this struct's value.
    /// 
    /// The unit and meaning of this value depends on the use case.
    pub fn edit_raw(&mut self, f: impl FnOnce(&mut Vec2)) {
        f(&mut self.raw)
    }
}

impl From<Vec2> for Size2 {
    fn from(value: Vec2) -> Self {
        Self { 
            x: SizeUnit::Pixels, 
            y: SizeUnit::Pixels,
            raw: value
        }
    }
}
