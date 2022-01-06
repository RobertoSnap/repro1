// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::DefaultPlugins;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Hosting,
}

fn main() {
    let mut app = App::new();
    app
        // .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins)
        .add_system_set(
            SystemSet::on_enter(GameState::Menu)
                .with_system(setup_menu)
                .label("container"),
        )
        .add_state(GameState::Menu)
        .add_system_set(
            SystemSet::on_enter(GameState::Menu)
                .with_system(change_state)
                .after("container"),
        )
        .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup_menu));

    app.run();
}

#[derive(Component)]
pub struct MenuContainer;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct HostButton;

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("started setup_menu");
    commands.spawn_bundle(UiCameraBundle::default());
    let parent = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgb(0.15, 0.15, 0.50).into(),
            ..Default::default()
        })
        .insert(MenuContainer)
        .id();
    println!("parent spawned: {:?}", parent);
    // play button
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgb(0.2, 0.2, 0.2).into(),
            ..Default::default()
        })
        .insert(Parent(parent))
        .insert(PlayButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Play".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });

    //host button
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgb(0.2, 0.2, 0.2).into(),
            ..Default::default()
        })
        .insert(Parent(parent))
        .insert(HostButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Host".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
    println!("children spawned");
    println!("ended setup_menu")
}

fn change_state(mut state: ResMut<State<GameState>>) {
    println!("started change_state");
    state.set(GameState::Hosting).unwrap();
    println!("ended change_state");
}

fn cleanup_menu(mut commands: Commands, mut query: Query<Entity, With<MenuContainer>>) {
    println!("Started cleanup");
    if let Ok(entity) = query.get_single_mut() {
        println!("entity {:?}", entity);
        commands.entity(entity).despawn_recursive();
    }
    println!("Ended cleanup");
}
