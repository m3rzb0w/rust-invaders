use bevy::prelude::*;

// region:      --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

const SPRITE_SCALE: f32 = 0.5;

// endregion:   --- Asset Constants


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .insert_resource(WindowDescriptor {
        title: "Rust Invaders!".to_string(),
        width: 598.0,
        height: 676.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup_system)
    .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());


    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // position window
    window.set_position(IVec2::new(2780, 49000));

    // add rectangle or player
    let bottom = -win_h / 2.;
    commands.spawn_bundle(SpriteBundle {
        // sprite: Sprite {
        //     color: Color::rgb(0.25, 0.25, 0.75),
        //     custom_size: Some(Vec2::new(150., 150.)),
        //     ..Default::default()
        // },
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}