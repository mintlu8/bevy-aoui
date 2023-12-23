//! A simple test case for percentage size.
use bevy::prelude::*;
use bevy_aoui::AouiPlugin;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AouiPlugin)
        .add_systems(Startup, init)
        .run();
}

pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());

    rectangle!((commands, assets) {
        anchor: CenterLeft,
        dimension: size2!(50%, 100%),
        color: color!(red),
        child: rectangle! {
            anchor: TopCenter,
            dimension: size2!(100%, 25%),
            color: color!(orange),
        },
        child: rectangle! {
            anchor: BottomCenter,
            dimension: size2!(100%, 25%),
            color: color!(purple),
        }
    });
    rectangle!((commands, assets) {
        anchor: CenterRight,
        dimension: size2!(50%, 100%),
        color: color!(blue)
    });
}