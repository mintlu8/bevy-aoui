//! This showcases discrete scrolling.

use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::{AouiPlugin, widgets::scroll::ScrollDiscrete};

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, init)
        .add_plugins(AouiPlugin)
        // classic macos stuff
        .run();
}


pub fn init(mut commands: Commands) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: fps_signal::<SigText>(|x: f32| format!("FPS: {:.2}", x))
    });

    let s = "abcdefghijklmnopqrstuvwxyz".chars();
    
    use_opacity(||vbox! (commands {
        dimension: [200, 60],
        hitbox: Rect(1),
        event: EventFlags::MouseWheel,
        range: 0..5,
        font_size: em(4),
        extra: ScrollDiscrete::new(),
        child: #text! {
            text: #s
        }
    }));
}

    
