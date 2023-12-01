use bevy::{prelude::*, utils::HashMap};
use bevy_aoui::AoUIPlugin;
use bevy_aoui_widgets::{AoUIExtensionsPlugin, widgets::richtext::{RichTextBuilder, FontStyle}};

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
        .add_plugins(AoUIPlugin)
        .add_plugins(AoUIExtensionsPlugin)
        .run();
}


pub fn init(mut commands: Commands, assets: Res<AssetServer>) {
    use bevy_aoui_widgets::dsl::prelude::*;
    commands.spawn(Camera2dBundle::default());
    let rich = paragraph! ((commands) {
        dimension: [600, 600],
        //margin: [8, 0],
        font_size: 32,
    });
    let mut builder = RichTextBuilder::new(&mut commands, HashMap::from([
        (("comicneue", FontStyle::None), assets.load("ComicNeue-Regular.ttf")),
        (("comicneue", FontStyle::Bold), assets.load("ComicNeue-Bold.ttf")),
        (("comicneue", FontStyle::Italic), assets.load("ComicNeue-Italic.ttf")),
        (("comicneue", FontStyle::Bold|FontStyle::Italic), assets.load("ComicNeue-BoldItalic.ttf")),
        (("roboto", FontStyle::None), assets.load("RobotoCondensed.ttf")),
        (("roboto", FontStyle::Bold), assets.load("RobotoCondensed-Bold.ttf")),
        (("roboto", FontStyle::Italic), assets.load("RobotoCondensed-Italic.ttf")),
        (("roboto", FontStyle::Bold|FontStyle::Italic), assets.load("RobotoCondensed-BoldItalic.ttf")),
    ]))
        .configure_size(assets.load("ComicNeue-Regular.ttf"), 32.0)
        //.with_size(32.0)
        .with_font("roboto")
        .with_color(Color::WHITE);
        //.with_ignore_space(true);

    builder.push_str(r#"
Hello, {orange:Rustaceans!}

Let's make out text {red:red} and {blue:blue.}

We can make it **bold** or *italic,* even {red:red and ***bold and italic.***}


We can use a different {@comicneue: font}.

We can align our text

left, {center:center,} {right:or right}

We can make our font {*2:bigger} or {*0.5:smaller}.

Let's permanantly change our font to {@comicneue} ComicNeue,
and use the color {green} green,
{right} and right align everything.

Awesome {br} right?
"#);

    let children = builder.build();
    commands.entity(rich).push_children(&children);
}
