use bevy::{prelude::*, text::BreakLineOn};
use crate::main_menu::{
    components::*,
    styles::*,
};



/*
    -------------------------
    ---- Spawn Main Menu ----
    -------------------------
*/

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    build_main_menu(&mut commands, &asset_server);
}




/*
    ---------------------------
    ---- Despawn Main Menu ----
    ---------------------------
*/

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_q: Query<Entity, With<MainMenu>>,
) {
    if let Ok(mm_entity) = main_menu_q.get_single() {
        commands.entity(mm_entity).despawn_recursive();
    }
}




/*
    -------------------------
    ---- Build Main Menu ----
    -------------------------
*/

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let mm_entity = commands.spawn((

        NodeBundle {
            style: MAIN_MENU_STYLE,
            ..default()
        },

        MainMenu {},

    ))
    .with_children(|parent| {

        // --- Title ---
        parent.spawn(
            NodeBundle {
                style: TITLE_STYLE,
                ..default()
            }
        ).with_children(|parent| {

            // Image 1
            parent.spawn(
                ImageBundle {
                    style: IMAGE_STYLE,
                    image: asset_server.load("sprites/ball_blue_large.png").into(),
                    ..default()
                }
            );

            // Text
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Bevy Ball Game",
                        get_title_text_style(&asset_server),
                    )],
                    justify: JustifyText::Center,
                    linebreak_behavior: BreakLineOn::NoWrap,
                },
                ..default()
            });

            // Image 2
            parent.spawn(
                ImageBundle {
                    style: IMAGE_STYLE,
                    image: asset_server.load("sprites/ball_red_large.png").into(),
                    ..default()
                }
            );
            
        });

        // --- Play ---
        parent.spawn(
            (
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                PlayButton {},
            )
        ).with_children(|parent| {
            // Play Button Text
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Play",
                                get_button_text_style(&asset_server)
                            )
                        ],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });
        
        // --- Quit ---
        parent.spawn(
            (
                ButtonBundle {
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                QuitButton {},
            )
        ).with_children(|parent| {
            // Quit Button Text
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Quit",
                                get_button_text_style(&asset_server),
                            )
                        ],
                        justify: JustifyText::Center,
                        ..default()
                    },
                    ..default()
                }
            );
        });

    })
    .id();

    mm_entity
}