use bevy::{
    prelude::*,
    text::BreakLineOn,
};


use crate::game::{
    score::resources::HighScores,
    ui::game_over::{
        components::*,
        styles::*,
    },
};




/* 
    -------------------------
    ---- Spawn Game Over ----
    -------------------------
*/

pub fn spawn_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    high_scores: Res<HighScores>,
) {
    commands.spawn((
        NodeBundle {
            style: GAME_OVER_MENU_STYLE,
            z_index: ZIndex::Local(2),
            ..default()
        },
        GameOverMenu {}
    )).with_children(|parent| {
        parent.spawn(
            NodeBundle {
                style: GAME_OVER_MENU_CONTAINER_STYLE,
                background_color: BACKGROUND_COLOR.into(),
                ..default()
            }
        ).with_children(|parent| {
            // Title
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Game Over!",
                        get_title_text_style(&asset_server),
                    )],
                    justify: JustifyText::Center,
                    linebreak_behavior: BreakLineOn::NoWrap,
                },
                ..default()
            });

            // Final Score Text
            if let Some(final_score) = high_scores.scores.last() {
                parent.spawn(
                    (
                        TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection::new(
                                        // "Final Score: ",
                                        format!("Final Score: {}", final_score.1),
                                        get_final_score_text_style(&asset_server),
                                    )
                                ],
                                justify: JustifyText::Center,
                                linebreak_behavior: BreakLineOn::NoWrap,
                            },
                            ..default()
                        },
                        FinalScoreText {}
                    )
                );
            }

            // Restart Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    RestartButton {}
                )
            ).with_children(|parent| {
                parent.spawn(TextBundle {
                    style: Style {..default()},
                    text: Text {
                        sections: vec![TextSection::new(
                            "Restart",
                            get_button_text_style(&asset_server)
                        )],
                        justify: JustifyText::Center,
                        linebreak_behavior: BreakLineOn::NoWrap,
                    },
                    ..default()
                });
            });

            // Main Menu Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MainMenuButton {},
                )
            ).with_children(|parent| {
                parent.spawn(TextBundle {
                    style: Style {..default()},
                    text: Text {
                        sections: vec![TextSection::new(
                            "Main Menu",
                            get_button_text_style(&asset_server),
                        )],
                        justify: JustifyText::Center,
                        linebreak_behavior: BreakLineOn::NoWrap,
                    },
                    ..default()
                });
            });

            // Quit Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    QuitButton {}
                )
            ).with_children(|parent| {
                parent.spawn(TextBundle {
                    style: Style {..default()},
                    text: Text {
                        sections: vec![TextSection::new(
                            "Quit",
                            get_button_text_style(&asset_server),
                        )],
                        justify: JustifyText::Center,
                        linebreak_behavior: BreakLineOn::NoWrap,
                    },
                    ..default()
                });
            });
        });
    });
}




/*
    --------------------------------
    ---- Despawn Game Over Menu ----
    --------------------------------
*/

pub fn despawn_game_over(
    mut commands: Commands,
    game_over_q: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(m_entity) = game_over_q.get_single() {
        commands.entity(m_entity).despawn_recursive();
    }
}