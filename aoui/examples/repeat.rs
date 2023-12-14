//! Demo for the span based layouts.

use bevy::prelude::*;
use bevy_aoui::AoUIPlugin;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_plugins(AoUIPlugin)
        .run();
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    let directions = [PI, 0.0, PI, 0.0, PI];
    let colors = colors!(blue100, blue200, blue300, blue400, blue500, blue600, blue700, blue800, blue900);
    let rotations = [-0.4, -0.3, -0.2, -0.1, 0.0, 0.1, 0.2, 0.3, 0.4];

    hbox! ((commands, assets) {
        child: #vbox! {
            rotation: #directions,
            child: #rectangle! {
                dimension: [40, 20],
                color: #colors,
                rotation: #rotations,
                z: -1,
            },
        },
    });
}