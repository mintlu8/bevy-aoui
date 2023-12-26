use bevy::math::Affine3A;
use bevy::sprite::{Sprite, TextureAtlasSprite};
use bevy::text::{TextLayoutInfo, Text2dBounds};
use bevy::prelude::*;

use bevy::sprite::Anchor as BevyAnchor;
use crate::{RotatedRect, BuildTransform, Dimension, DimensionSize, Transform2D, Opacity, OpacityWriter, BuildMeshTransform, Anchor};


/// Copy our `anchor` component's value to the `Anchor` component
pub fn copy_anchor(mut query: Query<(&mut BevyAnchor, &Transform2D)>) {
    query.par_iter_mut().for_each(|(mut a, anc)| *a = anc.anchor.into())
}

/// Copy evaluated `TextLayoutInfo` value to our `Dimension::Copied` value
pub fn copy_dimension_text(mut query: Query<(&TextLayoutInfo, &mut Dimension)>) {
    //let scale_factor = window.get_single().map(|x| x.scale_factor() as f32).unwrap_or(1.0);
    query.par_iter_mut().for_each(|(text, mut dim)| {
        match dim.dim {
            DimensionSize::Copied => dim.size = text.logical_size,
            _ => (),
        }
    })
}

/// Copy our `Anchors` component's value to the `Sprite` component
pub fn copy_anchor_sprite(mut query: Query<(&mut Sprite, &Transform2D)>) {
    query.par_iter_mut().for_each(|(mut sp, anc)| {
        sp.anchor = anc.anchor.into();
    })
}

/// Synchonize size between `Sprite` and `Dimension`
pub fn copy_dimension_sprite(mut query: Query<(&Sprite, &Handle<Image>, &mut Dimension)>, assets: Res<Assets<Image>>) {
    query.par_iter_mut().for_each(|(sp, im, mut dimension)| {
        dimension.update_size(|| {
            match sp.custom_size {
                Some(x) => x,
                None => match sp.rect {
                    Some(rect) => rect.max - rect.min,
                    None => assets.get(im).map(|x|x.size().as_vec2()).unwrap_or(Vec2::ZERO),
                },
            }
        });
    })
}

/// Copy anchor to the `TextureAtlasSprite` component
pub fn copy_anchor_atlas(mut query: Query<(&mut TextureAtlasSprite, &Transform2D)>) {
    query.par_iter_mut().for_each(|(mut sp, anc)| {
        sp.anchor = anc.anchor.into();
    })
}

/// copy size between `TextureAtlasSprite` to `Dimension`
pub fn copy_dimension_atlas(mut query: Query<(&TextureAtlasSprite, &Handle<TextureAtlas>, &mut Dimension)>, assets: Res<Assets<TextureAtlas>>) {
    query.par_iter_mut().for_each(|(sp, im, mut dimension)| {
        dimension.update_size(|| {
            match sp.custom_size {
                Some(size) => size,
                None => (|| -> Option<_> {
                   let rect = assets.get(im)?.textures.get(sp.index)?;
                   Some(rect.max - rect.min)
                })().unwrap_or(Vec2::ZERO)
            }
        });
    })
}

/// Synchonize size from `Dimension` to `Sprite`
pub fn sync_dimension_sprite(mut query: Query<(&mut Sprite, &Dimension)>) {
    query.par_iter_mut().for_each(|(mut sp, dimension)| {
        dimension.run_if_owned(|size| sp.custom_size = Some(size))
    })
}

/// Synchonize size from `Dimension` to `TextureAtlasSprite`
pub fn sync_dimension_atlas(mut query: Query<(&mut TextureAtlasSprite, &Dimension)>) {
    query.par_iter_mut().for_each(|(mut sp, dimension)| {
        dimension.run_if_owned(|size| sp.custom_size = Some(size))
    })
}

/// Opts out of synchronizing text bounds.
#[derive(Debug, Component)]
pub struct OptOutTextBoundsSync;

/// Opts out of synchronizing font size.
#[derive(Debug, Component)]
pub struct OptOutFontSizeSync;


/// Copy owned dimension as text bounds. 
pub fn sync_dimension_text_bounds(mut query: Query<(&mut Text2dBounds, &Dimension), Without<OptOutTextBoundsSync>>) {
    query.par_iter_mut().for_each(|(mut sp, dimension)| {
        if sp.as_ref().size != dimension.size {
            dimension.run_if_owned(|size| sp.size = size)
        }
    })
}



/// Copy em as text size.
pub fn set_occluded(mut query: Query<&mut Opacity>) {
    query.par_iter_mut().for_each(|mut op| { op.occluded = true })
}

/// Copy em as text size.
pub fn sync_em_text(mut query: Query<(&mut Text, &Dimension), Without<OptOutFontSizeSync>>) {
    query.par_iter_mut().for_each(|(mut sp, dimension)| {
        if sp.as_ref().sections.iter().any(|x| x.style.font_size != dimension.em) {
            sp.sections.iter_mut().for_each(|x| x.style.font_size = dimension.em)
        }
    })
}

/// Copy opacity as text alpha.
pub fn sync_opacity_text(mut query: Query<(&Opacity, &mut Text), With<OpacityWriter>>) {
    query.par_iter_mut().for_each(|(opacity, mut text)| {
        text.sections.iter_mut().for_each(|x| {x.style.color.set_a(opacity.get());} )
    })
}

/// Copy opacity as sprite alpha.
pub fn sync_opacity_sprite(mut query: Query<(&Opacity, &mut Sprite), With<OpacityWriter>>) {
    query.par_iter_mut().for_each(|(opacity, mut sprite)| {
        sprite.color.set_a(opacity.get());
    })
}

/// Copy opacity as atlas alpha.
pub fn sync_opacity_atlas(mut query: Query<(&Opacity, &mut TextureAtlasSprite), With<OpacityWriter>>) {
    query.par_iter_mut().for_each(|(opacity, mut sprite)| {
        sprite.color.set_a(opacity.computed_opacity);
    })
}

pub fn build_mesh_2d_global_transform(
    mut query: Query<(&RotatedRect, &Dimension, &mut GlobalTransform), With<BuildMeshTransform>>
) {
    query.iter_mut().for_each(|(rect, dim, mut transform)| 
        *transform = Affine3A::from_scale_rotation_translation(
            (rect.scale * dim.size).extend(1.0), 
            Quat::from_rotation_z(rect.rotation), 
            rect.anchor(Anchor::Center).extend(rect.z)
        ).into()
    );
}

/// Generate [`GlobalTransform`] with  [`BuildTransform`].
pub fn build_global_transform(
    mut query: Query<(&BuildTransform, &Transform2D, &RotatedRect, &mut GlobalTransform)>,
) {
    query.par_iter_mut().for_each(|(build, transform, rect, mut global)| {
        *global = Affine3A::from_scale_rotation_translation(
            rect.scale.extend(1.0), 
            Quat::from_rotation_z(rect.rotation), 
            rect.anchor(build.0.or(transform.anchor)).extend(rect.z)
        ).into()
    });
}
