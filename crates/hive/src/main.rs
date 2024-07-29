use apis::{Edge, HexCoordinate, HexOrientation, HexWorld, HexWorldShape, Hexagon};
use bevy::{color::palettes::css::{BLACK, BLUE, GREEN, GREY, LIGHT_BLUE, LIGHT_CYAN, MAGENTA, WHITE}, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_pancam::{PanCam, PanCamPlugin};

const FLAT_LABEL_OFFSETS: [(f32,f32);3] = [(0.7,0.0),(-0.7,0.7),(-0.7,0.0)];
const CELL_SIZE: f32 = 64.0;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanCamPlugin::default()))
        .add_systems(Startup, setup)
        // .add_systems(Update, giz)
        .run();
}

#[derive(Component)]
struct Tile;

fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>)
{
    commands.spawn(Camera2dBundle::default()).insert(PanCam::default());

    let world_shape = HexWorldShape::Hexagon(5);
    let indexer = apis::MapIndex::new(world_shape);
    let world =  HexWorld::new(HexOrientation::Flat, CELL_SIZE, world_shape);
    let mesh_shape = Hexagon::new(HexOrientation::Flat, CELL_SIZE - 1.0);

    let mesh_handle = meshes.add(mesh_shape);

    let font_size = CELL_SIZE / 3.0;

    let q_offset = HexOrientation::Flat.face_vec(Edge::Q.index()) * CELL_SIZE * 0.65;

    let r_offset = HexOrientation::Flat.face_vec(Edge::R.index()) * CELL_SIZE * 0.65;

    let s_offset = HexOrientation::Flat.face_vec(Edge::S.index()) * CELL_SIZE * 0.65;

    for i in 0..indexer.capacity() {
        let coords = indexer.coord(i);
        let translation = world.coord_to_world_v3(coords);
        let colour: Color = BLACK.into(); // Hsla::sequential_dispersed(i.try_into().unwrap()).into();
        let material = materials.add(ColorMaterial::from(colour));
        commands.spawn(MaterialMesh2dBundle {
            mesh: mesh_handle.clone().into(),
            material: material.clone(),
            transform: Transform::from_translation(translation),
            ..Default::default()
        }).insert(Tile);

        commands.spawn(Text2dBundle {
            text: Text::from_section(format!("{}", i), TextStyle { color: WHITE.into(), font_size, ..Default::default()}),
            transform: Transform::from_translation(translation.with_z(1.0)),
            ..Default::default()
        });

         commands.spawn(Text2dBundle {
             text: Text::from_section(format!("{}", coords.qrs().0), TextStyle { color: GREEN.into(), font_size, ..Default::default()}),
             transform: Transform::from_translation(translation + Vec3::new(q_offset.x, q_offset.y, 1.0)),
             ..Default::default()
         });

         commands.spawn(Text2dBundle {
             text: Text::from_section(format!("{}", coords.qrs().1), TextStyle { color: BLUE.into(), font_size, ..Default::default()}),
             transform: Transform::from_translation(translation + Vec3::new(r_offset.x, r_offset.y, 1.0)),
             ..Default::default()
         });

         commands.spawn(Text2dBundle {
             text: Text::from_section(format!("{}", coords.qrs().2), TextStyle { color: MAGENTA.into(), font_size, ..Default::default()}),
             transform: Transform::from_translation(translation + Vec3::new(s_offset.x, s_offset.y, 1.0)),
             ..Default::default()
         });
    }
}

fn giz(
    query: Query<&Transform, With<Tile>>,
    mut gizmos: Gizmos) {
        for &t in query.iter() {
            for k in 0..6 {
                let vec = HexOrientation::Flat.face_vec(k);
                let colour: Color = Hsla::sequential_dispersed(k.try_into().unwrap()).into();
                let o : Vec2 = Vec2::new(t.translation.x, t.translation.y);
                gizmos.arrow_2d(o, vec * CELL_SIZE, colour);
            }
        }
    }