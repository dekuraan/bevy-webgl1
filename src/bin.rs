use bevy::{prelude::*, render::options::WgpuOptions};
use bevy_obj::{ObjError, ObjPlugin};
use bevy_webgl1::Webgl1RenderingPlugin;
fn main() {
    console_error_panic_hook::set_once();
    App::new()
        .add_plugin(Webgl1RenderingPlugin)
        .insert_resource(WindowDescriptor {
            canvas: Some("#bevy".to_string()),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ObjPlugin)
        .add_startup_system(setup)
        .add_system(update_transform)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material_storage: ResMut<Assets<StandardMaterial>>,
) {
    let mesh: Handle<Mesh> = asset_server.load("window.obj");
    let texture_handle = asset_server.load("f-texture.png");
    let material_handle = material_storage.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        ..Default::default()
    });
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: material_handle.clone(),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn update_transform(mut query: Query<&mut Transform, Without<Camera>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_x(0.01));
    }
}
