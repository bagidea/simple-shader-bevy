use bevy::{
    prelude::*,
    sprite::Material2dPlugin,
    window::{
        EnabledButtons,
        PrimaryWindow
    }
};

use simple_shader_bevy::{
    components::WGSLMaterial,
    WinSize,
    DEFAULT_WINDOW_HEIGHT,
    DEFAULT_WINDOW_WIDTH
};

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "GLSL Example".to_string(),
            resolution: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT).into(),
            position: WindowPosition::Centered(MonitorSelection::Primary),
            resizable: false,
            enabled_buttons: EnabledButtons {
                maximize: false,
                minimize: false,
                ..default()
            },
            ..default()
        }),
        ..default()
    }))
    .add_plugins(Material2dPlugin::<WGSLMaterial>::default())
    .add_systems(Startup, setup_system)
    //.add_systems(Update, (update_system, window_resize_system))
    .add_systems(Update, update_system)
    .run();
}

fn setup_system(
    mut commands: Commands,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut meshed: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WGSLMaterial>>,

) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        window.position = WindowPosition::Centered(MonitorSelection::Current);

        let win_size: WinSize = WinSize { w: window.width(), h: window.height() };
        commands.insert_resource(win_size);

        commands.spawn(Camera2d);

        commands.spawn((
            Mesh2d(meshed.add(Rectangle::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT))),
            MeshMaterial2d(materials.add(WGSLMaterial {
                i_time: 0.,
                i_resolution: Vec3::new(window.width(), window.height(), 1.)
            })),
            Transform::from_scale(Vec3::ONE)
        ));
    }
}

fn update_system(
    time: Res<Time>,
    //mut query: Query<(&MeshMaterial2d<WGSLMaterial>, &mut Transform)>,
    query: Query<&MeshMaterial2d<WGSLMaterial>>,
    mut materials: ResMut<Assets<WGSLMaterial>>,
    win_size: Res<WinSize>
) {
    //for (material, mut transform) in query.iter_mut() {
    for material in query.iter() {
        if let Some(_material) = materials.get_mut(material) {
            _material.i_time = time.elapsed_secs() as f32;
            _material.i_resolution = Vec3::new(win_size.w, win_size.h, 1.);
        }
    
        //transform.scale = Vec3::new(win_size.w / DEFAULT_WINDOW_WIDTH, win_size.h / DEFAULT_WINDOW_HEIGHT, 1.);
    }
}

/*fn window_resize_system(
    mut win_size: ResMut<WinSize>,
     query: Query<&Window>
) {
    for window in query.iter() {
        win_size.w = window.width();
        win_size.h = window.height();
    }
}*/