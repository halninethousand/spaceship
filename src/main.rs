mod debug;
mod movement;
mod spaceship;
mod camera;
mod asteroids;
mod asset_loader;

use bevy::prelude::*;
use bevy::render::*;
use bevy::render::settings::*;
use debug::DebugPlugin;
use camera::CameraPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
use asteroids::AsteroidPlugin;


fn main () {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.0,
        })
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DebugPlugin)
		.add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
            backends:Some(Backends::VULKAN),
                    ..default()
               }),
               synchronous_pipeline_compilation: false,
       }))
        .run();
}