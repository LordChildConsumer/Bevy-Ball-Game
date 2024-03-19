use bevy::prelude::*;

// HACK: This whole script. I don't know why but without it 'asset_server.load()' won't work more than once.


#[derive(Resource)]
pub struct PlayerSprite(pub Handle<Image>);

#[derive(Resource)]
pub struct EnemySprite(pub Handle<Image>);



pub fn load_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Player Sprite
    commands.insert_resource(PlayerSprite(asset_server.load("sprites/ball_blue_large.png")));

    // Enemy Sprite
    commands.insert_resource(EnemySprite(asset_server.load("sprites/ball_red_large.png")));
}