use bevy::{prelude::{Plugin, PostUpdate, IntoSystemConfigs, Update}, ecs::schedule::{SystemSet, IntoSystemSetConfigs}, app::PreUpdate};
use crate::{schedule::{AoUIStoreOutput, AoUILoadInput}, WorldExtension};

use crate::events::AoUIEventSet;
use crate::util::{Submit, Change};

use super::{inputbox, button::{self, CursorDefault}, drag::{self, drag_start, DragSignal}, richtext, scroll, scrollframe};




#[derive(SystemSet, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct AoUIWidgetsEventSet;

pub(crate) struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<CursorDefault>()
            .configure_sets(PreUpdate, AoUIWidgetsEventSet.after(AoUIEventSet))
            .add_systems(PreUpdate, bevy::ecs::prelude::apply_deferred
                .after(AoUIEventSet)
                .before(AoUIWidgetsEventSet))
            .add_systems(PreUpdate, (
                inputbox::text_on_mouse_down,
                inputbox::text_on_click_outside,
                inputbox::text_on_mouse_double_click,
                inputbox::inputbox_keyboard,
                button::propagate_focus,
            ).in_set(AoUIWidgetsEventSet))
            .add_systems(Update, (
                inputbox::update_inputbox_cursor,
                inputbox::format_signal::<Submit>,
                inputbox::format_signal::<Change>,
                button::set_cursor,
                button::event_conditional_visibility,
                button::state_conditional_visibility,
                drag::drag_start,
                drag::drag_end,
                drag::dragging.after(drag_start),
                scroll::scrolling,
                scrollframe::clipping_layer,
            ))
            .add_systems(PostUpdate, drag::apply_constraints.in_set(AoUILoadInput))
            .add_systems(PostUpdate, richtext::synchronize_glyph_spaces.in_set(AoUILoadInput))
            .add_systems(PostUpdate, inputbox::sync_em_inputbox.in_set(AoUIStoreOutput))
            .register_signal::<DragSignal>()
        ;
    }
}