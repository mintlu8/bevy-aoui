use macroex::*;
use macroex_extras::*;
use proc_macro2::*;
use quote::{quote, format_ident, ToTokens};

pub struct Length(pub Ident, pub f32);

ident_validator!(XLit "x");
ident_validator!(PxLit "px");
ident_validator!(EmLit "em");
ident_validator!(RemLit "rem");

impl FromMacro for Length {
    fn from_one(tt: TokenTree) -> Result<Self, Error> {
        let Number(v) = Number::from_one(tt)?;
        Ok(Self(format_ident!("Pixels"), v))
    }

    fn from_many(tokens: TokenStream) -> Result<Self, Error> {
        let mut iter = tokens.into_iter();
        let Number(v) = iter.extract()?;
        let ident = match iter.extract()? {
            Either4::A(PxLit) => format_ident!("Pixels"),
            Either4::B(EmLit) => format_ident!("Pixels"),
            Either4::C(RemLit) => format_ident!("Pixels"),
            Either4::D(PunctOf::<'%'>) => format_ident!("Pixels"),
        };
        Ok(Self(ident, v))
    }
}

pub struct Size2([Length;2]);

impl FromMacro for Size2 {
    fn from_one(tt: TokenTree) -> Result<Self, Error> {
        let Repeat::<_, 2>(v) = Repeat::from_one(tt)?;
        Ok(Self(v))
    }
}

impl ToTokens for Size2 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let a = &self.0[0].0;
        let b = &self.0[0].1;
        let c = &self.0[1].0;
        let d = &self.0[1].1;
        quote!(::bevy_aoui::Size2::new(
            (::bevy_aoui::SizeUnit::#a, #b), 
            (::bevy_aoui::SizeUnit::#c, #d),
        )).to_tokens(tokens)
    }
}

impl Size2 {
    pub fn get_raw(&self) -> Option<Vec2> {
        if self.0[0].0 == format_ident!("px") && self.0[1].0 == format_ident!("px") {
            Some(Vec2([self.0[0].1, self.0[1].1]))
        } else {
            None
        }
    }
}

pub struct SetEM(Ident, [f32; 2]);

impl FromMacro for SetEM {
    fn from_one(tt: TokenTree) -> Result<Self, Error> {
        let Repeat::<_, 2>(NumberList(list)) = Repeat::from_one(tt)?;
        Ok(Self(format_ident!("Pixels"), list))
    }

    fn from_many(tokens: TokenStream) -> Result<Self, Error> {
        let mut iter = tokens.into_iter();
        match iter.extract()? {
            Either::A(XLit) => {
                let Repeat::<_, 2>(NumberList(list)) = iter.extract()?;
                Ok(Self(format_ident!("Mult"), list))
            }
            Either::B(Repeat::<_, 2>(NumberList(list))) => {
                let RemLit = iter.extract()?;
                Ok(Self(format_ident!("MultRem"), list))
            }
        }
    }
}

impl ToTokens for SetEM {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.0;
        let [a, b] = self.1;
        quote!(::bevy_aoui::SetEM::#ident(
            ::bevy::prelude::Vec2(#a, #b)
        )).to_tokens(tokens)
    }
}

#[derive(Debug, FromMacro)]
pub struct Rect {
    pub dimension: Option<NumberList<[f32; 2]>>,
    pub center: Option<NumberList<[f32; 2]>>,
    pub min: Option<NumberList<[f32; 2]>>,
    pub max: Option<NumberList<[f32; 2]>>,
}

macroex::call_syntax!(
    "::bevy::prelude::Vec2::new($)", 
    #[derive(Debug, Clone, Copy)]
    pub Vec2(pub [f32; 2])
);

#[derive(Debug, FromMacro)]
#[macroex(rename_all = "lowercase")]
pub enum HitboxShape {
    Rectangle,
    Ellipse,
}

impl ToTokens for HitboxShape {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            HitboxShape::Rectangle => {
                quote!(::bevy_aoui::HitboxShape::Rectangle)
                    .to_tokens(tokens)
            },
            HitboxShape::Ellipse => {
                quote!(::bevy_aoui::HitboxShape::Ellipse)
                    .to_tokens(tokens)
            },
        }
    }
}

#[derive(FromMacro)]
pub struct Hitbox {
    #[macroex(required)]
    pub shape: HitboxShape,
    #[macroex(default="MaybeExpr::Value(NumberList(Vec2([1.0, 1.0])))")]
    pub size: MaybeExpr<NumberList<Vec2>>,
}


#[derive(Debug)]
pub struct AnchorEntry(Either<Ident, [f32; 2]>);

impl FromMacro for AnchorEntry {
    fn from_one(tt: TokenTree) -> Result<Self, Error> {
        Ok(Self(Either::from_one(tt)?))
    }
}


impl ToTokens for AnchorEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.0 {
            Either::A(ident) => 
                quote!(::bevy::sprite::Anchor::#ident).to_tokens(tokens),
            Either::B([x,y]) => 
                quote!(::bevy::sprite::Anchor::Custom(::bevy::prelude::Vec2::new(#x, #y))).to_tokens(tokens),
        }
    }
}

#[derive(Debug, FromMacro)]
pub enum Linebreak {
    #[macroex(rename="linebreak")]
    Linebreak,
    #[macroex(rename="self")]
    BreakOnSelf,
}

#[derive(Debug, FromMacro)]
pub struct Mat2{
    pub rotation: f32,
    #[macroex(default = "[1.0, 1.0]")]
    pub scale: [f32; 2],
}

#[derive(Debug)]
pub struct Color(pub Rgba<[f32; 4]>);


impl FromMacro for Color {
    fn from_one(tt: TokenTree) -> Result<Self, Error> {
        Ok(Self(Rgba::from_one(tt)?))
    }
}

impl ToTokens for Color {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Color(Rgba([r,g,b,a])) = self;
        quote!(::bevy::prelude::Color::RgbaLinear {
            red: #r, green: #g, blue: #b, alpha: #a
        }.as_rgba()).to_tokens(tokens)
    }
}
