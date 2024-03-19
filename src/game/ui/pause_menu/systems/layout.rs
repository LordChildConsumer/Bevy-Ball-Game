use bevy::{
    prelude::*,
    text::BreakLineOn,
};

use crate::game::ui::pause_menu::{
    components::*,
    styles::*,
};




/*
    --------------------------
    ---- Spawn Pause Menu ----
    --------------------------
*/

pub fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        (
            NodeBundle {
                style: PAUSE_MENU_STYLE,
                z_index: ZIndex::Local(1),
                ..default()
            },
            PauseMenu{}
        )
    ).with_children(|parent| {

        parent.spawn(
            NodeBundle {
                style: PAUSE_MENU_CONTAINER_STYLE,
                background_color: BACKGROUND_COLOR.into(),
                ..default()
            }
        ).with_children(|parent| {

            // Title
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Paused",
                                get_title_text_style(&asset_server),
                            )
                        ],
                        justify: JustifyText::Center,
                        linebreak_behavior: BreakLineOn::NoWrap,
                    },
                    ..default()
                }
            );

            // Resume Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ResumeButton {}
                )
            ).with_children(|parent| {

                parent.spawn(
                    TextBundle {
                        style: Style { ..default() },
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "Resume",
                                    get_button_text_style(&asset_server)
                                )
                            ],
                            justify: JustifyText::Center,
                            linebreak_behavior: BreakLineOn::NoWrap,
                        },
                        ..default()
                    }
                );
            });

            // Main Menu Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MainMenuButton {}
                )
            ).with_children(|parent| {

                parent.spawn(
                    TextBundle {
                        style: Style { ..default() },
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "Main Menu",
                                    get_button_text_style(&asset_server),
                                )
                            ],
                            justify: JustifyText::Center,
                            linebreak_behavior: BreakLineOn::NoWrap,
                        },
                        ..default()
                    }
                );

            });

            // Quit Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: BACKGROUND_COLOR.into(),
                        ..default()
                    },
                    QuitButton {}
                )
            ).with_children(|parent| {

                parent.spawn(
                    TextBundle {
                        style: Style { ..default() },
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "Quit",
                                    get_button_text_style(&asset_server)
                                )
                            ],
                            justify: JustifyText::Center,
                            linebreak_behavior: BreakLineOn::NoWrap,
                        },
                        ..default()
                    }
                );

            });

        });

    });
}




/*
    ----------------------------
    ---- Despawn Pause Menu ----
    ----------------------------
*/

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_q: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pm_entity) = pause_menu_q.get_single() {
        commands.entity(pm_entity).despawn_recursive();
    }
}