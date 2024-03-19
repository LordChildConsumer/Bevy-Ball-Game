use bevy::prelude::*;




#[derive(Resource)]
pub struct PlayerSprite(pub Handle<Image>);

#[derive(Resource)]
pub struct EnemySprite(pub Handle<Image>);

#[derive(Resource)]
pub struct StarSprite(pub Handle<Image>);


pub fn load_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Player Sprite
    commands.insert_resource(PlayerSprite(asset_server.load("sprites/ball_blue_large.png")));

    // Enemy Sprite
    commands.insert_resource(EnemySprite(asset_server.load("sprites/ball_red_large.png")));

    // Star Sprite
    commands.insert_resource(StarSprite(asset_server.load("sprites/star.png")));
}




#[derive(Resource)]
pub struct EnemyBounceSound1(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct EnemyBounceSound2(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct StarCollectSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct PlayerDeathSound(pub Handle<AudioSource>);


pub fn load_sounds(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Enemy Bounce 1
    commands.insert_resource(EnemyBounceSound1(asset_server.load("sounds/enemy_bounce_1.ogg")));

    // Enemy Bounce 2
    commands.insert_resource(EnemyBounceSound2(asset_server.load("sounds/enemy_bounce_2.ogg")));

    // Star Collect
    commands.insert_resource(StarCollectSound(asset_server.load("sounds/collect_star.ogg")));

    // Player Death
    commands.insert_resource(PlayerDeathSound(asset_server.load("sounds/player_death.ogg")));
}