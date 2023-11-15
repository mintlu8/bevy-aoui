use bevy::{math::Vec2, sprite::{Anchor, Mesh2dHandle, ColorMaterial}, prelude::{Color, Handle, Commands, Entity}};
use bevy_aoui::{Size2, SetEM, Hitbox, bundles::{AoUIBundle, BuildGlobalBundle}, ScaleErase};
use bevy_prototype_lyon::prelude::*;

use crate::{dsl::{prelude::*, core::{transform2d, dimension, common_plugins}}, widgets::shape::{Shapes, ShapeDimension}};

use super::{convert::DslInto, AoUIWidget};

impl DslInto<Option<Fill>> for Color{
    fn dinto(self) -> Option<Fill> {
        Some(Fill::color(self))
    }
}

impl DslInto<Option<Stroke>> for (Color, f32){
    fn dinto(self) -> Option<Stroke> {
        let (color, size) = self;
        Some(Stroke { 
            color, 
            options: StrokeOptions::DEFAULT
                .with_line_width(size)
                .with_start_cap(LineCap::Round)
                .with_end_cap(LineCap::Round)
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct Shape {
    pub anchor: Anchor,
    pub parent_anchor: Option<Anchor>,
    pub center: Option<Anchor>,
    pub visible: Option<bool>,
    pub offset: Size2,
    pub rotation: f32,
    pub scale: Option<OneOrTwo<Vec2>>,
    pub z: f32,
    pub dimension: Option<Size2>,
    /// Initialize render size, by default Vec2::ONE.
    pub size: Option<Vec2>,
    pub em: SetEM,
    pub linebreak: bool,
    pub hitbox: Option<Hitbox>,
    pub shape: Shapes,
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
    pub stroke_size: f32,
    //pub material: Option<Handle<Material2d>>,
    pub default_material: Handle<ColorMaterial>,
    /// Unlike the default behavior of `Lyon`,
    /// 
    /// The default is `Round`.
    pub caps: Option<OneOrTwo<[LineCap; 2]>>,
}

impl AoUIWidget for Shape {
    fn spawn_with(self, commands: &mut Commands) -> Entity {
        let shape = self.shape;
        let transform = transform2d!(self);
        let anchor = transform.anchor;
        let size = self.size.unwrap_or(Vec2::ONE);

        let mut base = commands.spawn((
            AoUIBundle {
                transform,
                dimension: dimension!(self),
                ..Default::default()
            },
            BuildGlobalBundle::default(),
            ScaleErase,
            shape.build_path(anchor, size),
            shape,
            ShapeDimension { size, anchor },
            Mesh2dHandle::default(),
            self.default_material,
        ));

        if let Some(fill) = self.fill {
            base.insert(fill);
        }
        if let Some(mut stroke) = self.stroke {
            if let Some(OneOrTwo([l ,r])) = self.caps.dinto() {
                stroke.options = stroke.options.with_start_cap(l).with_end_cap(r)
            }
            base.insert(stroke);
        }
        common_plugins!(self, base);
        base.id()
    }
}