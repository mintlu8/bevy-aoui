#![recursion_limit = "256"]
use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_aoui::WorldExtension;
use bevy_aoui::AouiPlugin;
use bevy_aoui::widgets::scroll::ScrollDirection;

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
        .register_cursor_default(CursorIcon::Arrow)
        .insert_resource(ScrollDirection::INVERTED)
        .run();
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    text!(commands {
        anchor: TopRight,
        text: "FPS: 0.00",
        color: color!(gold),
        extra: fps_signal::<SigText>(|x: f32| format!("FPS: {:.2}", x))
    });
    
    let (send, recv_rot, fold_recv) = signal();

    let elements = [
        "Water", "Earth", "Fire", "Air"
    ];

    let (text_ctx, text_recv) = radio_button_group::<_, 4>("");
    check_button!((commands, assets){
        dimension: size2!(22 em, 2 em),
        on_change: send,
        child: hspan! {
            dimension: size2!(22 em, 2 em),
            font_size: em(2),
            child: text! {
                anchor: Left,
                text: "Selected Element:",
                font: "ComicNeue-Bold.ttf",
            },
            child: text! {
                anchor: Left,
                text: "",
                font: "ComicNeue-Bold.ttf",
                extra: text_recv.new_receiver().map::<SigText>(|x: &str| x.to_string())
            },
        },
        child: text! {
            font_size: em(2),
            anchor: Right,
            center: Center,
            rotation: degrees(90),
            text: "v",
            extra: recv_rot.map::<SigRotation>(|x: bool| if x {0.0} else {PI/2.0}),
            extra: transition! (Rotation 0.5 CubicInOut default PI)
        },
        child: clipping_layer! {
            anchor: TopRight,
            parent_anchor: BottomRight,
            layer: 1,
            buffer: [800, 800],
            scroll: Scrolling::Y,
            extra: fold_recv.map::<SigOpacity>(|x: bool| if x {1.0f32} else {0.0f32}),
            extra: transition! (Opacity 0.5 Linear default 0.0),
            dimension: size2!(14 em, 4 em),
            child: use_opacity(|| vbox!((commands, assets){
                anchor: Top,
                child: #radio_button! {
                    dimension: size2!(14 em, 2 em),
                    context: #text_ctx,
                    value: #elements,
                    child: sprite!{
                        anchor: Left,
                        dimension: size2!(2 em, 2 em),
                        sprite: "radio.png",
                        extra: DisplayIf(CheckButtonState::Checked),
                    },
                    child: sprite!{
                        anchor: Left,
                        dimension: size2!(2 em, 2 em),
                        sprite: "unchecked.png",
                        extra: DisplayIf(CheckButtonState::Unchecked)
                    },
                    child: text!{
                        anchor: Left,
                        offset: size2!(2.5 em, 0),
                        text: #elements,
                    },
                },
            })),
        }
    });
}
