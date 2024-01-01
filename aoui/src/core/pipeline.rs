use bevy::{prelude::*, window::PrimaryWindow, ecs::query::{ReadOnlyWorldQuery, WorldQuery}, math::Affine2};

use crate::{*, layout::*, dimension::DimensionMut};

type AouiEntity<'t> = (
    Entity,
    DimensionMut,
    &'t Transform2D,
    &'t mut RotatedRect,
    &'t mut Opacity,
    &'t mut Clipping,
);

const Z_INCREMENT: f32 = 0.01;

#[allow(clippy::too_many_arguments)]
#[allow(clippy::needless_pass_by_ref_mut)]
fn propagate<TAll: ReadOnlyWorldQuery>(
    parent: ParentInfo,
    entity: Entity,
    rem: f32,
    mut_query: &mut Query<AouiEntity, TAll>,
    flex_query: &Query<&Container>,
    parent_query: &Query<&Parent>,
    child_query: &Query<&Children>,
    control_query: &Query<&LayoutControl>,
    queue: &mut Vec<(Entity, ParentInfo)>) {

    if !mut_query.contains(entity) { return; }

    if parent_query.get(entity).ok().map(|x| x.get()) != parent.entity {
        panic!("Malformed hierarchy, parent child mismatch.")
    }

    // SAFETY: safe since double mut access is gated by the hierarchy check
    let Ok((entity, mut dim, transform, mut orig, mut opacity, mut clipping, ..)) 
        = (unsafe {mut_query.get_unchecked(entity)}) else {return};

    let (dimension, em) = dim.update(parent.dimension, parent.em, rem);
    let offset = transform.offset.as_pixels(parent.dimension, em, rem);
    
    clipping.global = parent.clip;

    opacity.occluded = false;
    opacity.computed_opacity = opacity.opacity * parent.opacity;
    opacity.computed_disabled = opacity.disabled || parent.disabled;
    let disabled = opacity.computed_disabled;
    let opacity = opacity.computed_opacity;

    if let Ok(layout) = flex_query.get(entity) {
        let children = child_query.get(entity).map(|x| x.iter()).into_iter().flatten();
        let mut other_entities = Vec::new();
        let mut args = Vec::new();
        let mut index = 0;
        for child in children {
            if !mut_query.contains(*child) { continue }
            if parent_query.get(*child).ok().map(|x| x.get()) != Some(entity) {
                panic!("Malformed hierarchy, parent child mismatch.")
            }
            // otherwise cloned property will recursively overflow this entire thing.
            let dimension = if dim.is_owned() {dimension} else {Vec2::ZERO};

            let range = layout.range.clone().unwrap_or(0..usize::MAX);
            // SAFETY: safe since double mut access is gated by the hierarchy check
            if let Ok((_, mut child_dim, child_transform, ..)) = unsafe { mut_query.get_unchecked(*child) } {
                match control_query.get(*child) {
                    Ok(LayoutControl::IgnoreLayout) => other_entities.push((
                        *child, 
                        child_transform.get_parent_anchor()
                    )),
                    control => {
                        if range.contains(&index) {
                            let _ = child_dim.update(dimension, em, rem);
                            args.push(LayoutItem {
                                entity: *child,
                                anchor: child_transform.get_parent_anchor(),
                                dimension: child_dim.estimate(dimension, em, rem),
                                control: control.copied().unwrap_or_default(),
                            });
                        }
                        index += 1;
                    }
                };
            }
        }
        let margin = layout.margin.as_pixels(parent.dimension, em, rem);
        let LayoutOutput{ mut entity_anchors, dimension: size } = layout.place(
            &LayoutInfo { dimension, em, rem, margin }, 
            args
        );
        let padding = layout.padding.as_pixels(parent.dimension, em, rem) * 2.0;
        let fac = size / (size + padding);
        let size = size + padding;
        if !fac.is_nan() {
            entity_anchors.iter_mut().for_each(|(_, anc)| *anc *= fac);
        }
        dim.dynamic.size = size;
        if layout.dimension_agnostic() {
            dim.dynamic.reliable_size = size;
        } else {
            dim.dynamic.reliable_size = Vec2::ZERO;
        }
        let rect = RotatedRect::construct(
            &parent,
            transform.parent_anchor,
            transform.anchor,
            offset,
            size,
            transform.get_center(),
            transform.rotation,
            transform.scale,
            if transform.z != 0.0 {
                parent.rect.z + transform.z
            } else {
                parent.rect.z + Z_INCREMENT
            }        
        );

        let info = ParentInfo {
            entity: Some(entity),
            rect,
            anchor: None,
            dimension: size,
            em,
            opacity,
            disabled,
            clip: if clipping.clip {Some(rect.affine.inverse())} else {parent.clip},
        };

        queue.extend(entity_anchors.into_iter().map(|(e, anc)| (e, info.with_anchor(anc))));
        if orig.as_ref() != &rect {
            *orig = rect
        }
        for (child, _) in other_entities {
            queue.push((child, info))
        }
        return;
    }
    dim.dynamic.reliable_size = Vec2::ZERO;

    let rect = RotatedRect::construct(
        &parent,
        transform.parent_anchor,
        transform.anchor,
        offset,
        dimension,
        transform.get_center(),
        transform.rotation,
        transform.scale,
        if transform.z != 0.0 {
            parent.rect.z + transform.z
        } else {
            parent.rect.z + Z_INCREMENT
        }
    );
    

    if let Ok(children) = child_query.get(entity) {
        let info = ParentInfo {
            entity: Some(entity),
            rect,
            anchor: None,
            dimension,
            em,
            opacity,
            disabled,
            clip: if clipping.clip {Some(rect.affine.inverse())} else {parent.clip},
        };
        for child in children {
            queue.push((*child, info))
        }
    }

    if orig.as_ref() != &rect {
        *orig = rect
    }
}

/// Query for finding the root rectangle of a `compute_aoui_transforms` pass.
/// 
/// Usually `PrimaryWindow`.
pub trait RootQuery<'t> {
    type Query: WorldQuery;
    type ReadOnly: ReadOnlyWorldQuery;

    fn as_rect(query: &Query<Self::Query, Self::ReadOnly>) -> (RotatedRect, Vec2);
}

impl<'t> RootQuery<'t> for PrimaryWindow {
    type Query= &'t Window;
    type ReadOnly = With<PrimaryWindow>;

    fn as_rect(query: &Query<Self::Query, Self::ReadOnly>) -> (RotatedRect, Vec2) {
        let window = match query.get_single(){
            Ok(w) => w,
            Err(_) => return Default::default(), 
        };
        let dim = Vec2::new(window.width(), window.height());
        (RotatedRect {
            affine: Affine2::from_scale(dim),
            rotation: 0.0,
            scale: Vec2::ONE,
            z: 0.0
        }, dim)
    }
}

pub(crate) type TRoot = Without<Parent>;
pub(crate) type TAll = ();

/// The main computation step.
/// 
/// For custom usage,
/// 
/// R: Get root rectangle,
/// 
/// TRoot: Readonly query for child of root rectangle.
/// 
/// TAll: Readonly query for all children, including TRoot.
#[allow(clippy::too_many_arguments)]
pub fn compute_aoui_transforms<'t, R: RootQuery<'t>, TRoot: ReadOnlyWorldQuery, TAll: ReadOnlyWorldQuery>(
    root: Query<R::Query, R::ReadOnly>,
    root_entities: Query<Entity, TRoot>,
    mut entity_query: Query<AouiEntity, TAll>,
    flex_query: Query<&Container>,
    parent_query: Query<&Parent>,
    child_query: Query<&Children>,
    control_query: Query<&LayoutControl>,
    res_rem: Option<Res<AouiREM>>,
) {
    let rem = res_rem.map(|x| x.get()).unwrap_or(16.0);

    let (window_rect, dimension) = R::as_rect(&root);

    let mut queue = Vec::new();
    let window_info = ParentInfo {
        entity: None,
        rect: window_rect,
        anchor: None,
        dimension,
        em: rem,
        opacity: 1.0,
        disabled: false,
        clip: None,
    };

    for (entity, ..) in entity_query.iter_many(root_entities.iter()) {
        queue.push((entity, window_info))
    }

    while !queue.is_empty() {
        for (entity, parent) in std::mem::take(&mut queue) {
            propagate(parent, 
                entity, 
                rem, 
                &mut entity_query, 
                &flex_query, 
                &parent_query, 
                &child_query, 
                &control_query, 
                &mut queue
            );
        }
    }
}
