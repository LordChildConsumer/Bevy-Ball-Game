use bevy::{
    prelude::*,
    text::BreakLineOn,
};


use crate::game::ui::hud::{
    styles::*,
    components::*,
};



/*
    ------------------------
    ---- Spawn Game HUD ----
    ------------------------
*/

pub fn spawn_game_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        NodeBundle {
            style: HUD_STYLE,
            ..default()
        },
        Hud {}
    )).with_children(|parent| {
        // LHS
        parent.spawn(
            NodeBundle {
                style: LHS_STYLE,
                background_color: BACKGROUND_COLOR.into(),
                ..default()
            }
        ).with_children(|parent| {
            // Star
            parent.spawn(ImageBundle {
                style: IMAGE_STYLE,
                image: asset_server.load("sprites/star.png").into(),
                ..default()
            });

            // Score text
            parent.spawn((
                TextBundle {
                    style: Style {..default() },
                    text: Text {
                        sections: vec![TextSection::new(
                            "0",
                            get_text_style(&asset_server)
                        )],
                        justify: JustifyText::Center,
                        linebreak_behavior: BreakLineOn::NoWrap,
                    },
                    ..default()
                },
                ScoreText {},
            ));
        });
        // RHS
        parent.spawn(NodeBundle {
            style: RHS_STYLE,
            background_color: BACKGROUND_COLOR.into(),
            ..default()
        }).with_children(|parent| {
            // Enemy Text
            parent.spawn((
                TextBundle {
                    style: Style {..default()},
                    text: Text {
                        sections: vec![TextSection::new(
                            "0",
                            get_text_style(&asset_server),
                        )],
                        justify: JustifyText::Center,
                        linebreak_behavior: BreakLineOn::NoWrap,
                    },
                    ..default()
                },
                EnemyText {},
            ));

            // Enemy Image
            parent.spawn(ImageBundle {
                style: IMAGE_STYLE,
                image: asset_server.load("sprites/ball_red_large.png").into(),
                ..default()
            });
        });
    });
}




/*
    --------------------------
    ---- Despawn Game HUD ----
    --------------------------
*/

pub fn despawn_game_hud(
    mut commands: Commands,
    hud_q: Query<Entity, With<Hud>>,
) {
    for entity in hud_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}