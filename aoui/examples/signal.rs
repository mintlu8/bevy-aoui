use bevy::prelude::*;
use bevy_aoui::{AoUIPlugin, WorldExtension};


pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, init)
        .register_cursor_default(CursorIcon::Arrow)
        .add_plugins(AoUIPlugin)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    let (submit_sender, recv_s1, recv_s2) = signal();
    let (change_sender, recv_c1, recv_c2) = signal();
    inputbox! ((commands, assets) {
        dimension: size2!([400, 1 em]),
        offset: [0, 200],
        font_size: em(4),
        hitbox: Rect(1),
        text: "Type here and press Enter.",
        font: assets.load::<Font>("RobotoCondensed.ttf"),
        color: color!(red),
        cursor_bar: rectangle! {
            color: color!(gold),
            z: -0.1,
            dimension: size2!([2, 1 em]),
        },
        cursor_area: rectangle! {
            color: color!(green) * 0.5,
            z: -0.2,
            dimension: size2!([12, 1 em]),
        },
        submit: submit_sender,
        change: change_sender,
    });

    textbox!(commands {
        text: "This is a receiver.",
        offset: [-200, 0],
        font: assets.load::<Font>("RobotoCondensed.ttf"),
        extra: recv_s1.mark::<SigText>(),
    });

    textbox!(commands {
        text: "This is a formatter.",
        offset: [-200, -200],
        font: assets.load::<Font>("RobotoCondensed.ttf"),
        extra: recv_s2.mark::<SigText>().format("Received string \"{%}\"!"),
    });

    textbox!(commands {
        text: "This is a change detector.",
        offset: [200, 0],
        font: assets.load::<Font>("RobotoCondensed.ttf"),
        extra: recv_c1.mark::<SigText>(),
    });

    textbox!(commands {
        text: "This is a change detecting formatter.",
        offset: [200, -200],
        font: assets.load::<Font>("RobotoCondensed.ttf"),
        extra: recv_c2.mark::<SigText>().format("Received string \"{%}\"!"),
    });
}
