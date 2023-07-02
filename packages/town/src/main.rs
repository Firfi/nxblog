mod building_entrance_ref;
mod level_height;

use wasm_bindgen::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use building_entrance_ref::*;

use bevy::{prelude::*, winit::WinitSettings};
use bevy::log::Level;
use crate::level_height::{LevelHeight, set_level_height_to_current_level};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
  fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = window)]
  fn go_town(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
  alert("Hello, wasm-game-of-life!");
  main();
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(LdtkPlugin)
    .insert_resource(WinitSettings::game())
    .add_startup_system(setup)
    .add_startup_system(setup_ldtk)
    .add_system(resolve_building_entrance_references)
    .add_system(update_cursor_pos)
    .add_system(set_level_height_to_current_level)
    // .add_startup_system(button_setup)
    // .add_system(button_interaction_system)
    .insert_resource(LevelSelection::Index(0))
    .init_resource::<CursorPos>()
    .init_resource::<LevelHeight>()
    .register_ldtk_entity::<BuildingAreaBundle>("BuildingArea")
    .register_ldtk_entity::<BuildingEntranceBundle>("BuildingEntrance")
    .add_system(ls)
    // not needed; for egui inspector
    .register_type::<UrlPath>()
    .register_type::<BuildingEntranceRef>()
    .run();
}

fn ls (transforms_query: Query<(&LevelPositional, &BuildingEntranceRef)>,
  cursor_position_res: Res<CursorPos>,
  entrance_positional_query: Query<(&GridCoords, &BuildingEntrance)>
) {
  for entrance in transforms_query.iter().filter(|e| {
    let gpxx_min = e.0.px.x;
    let gpxx_max = gpxx_min + e.0.width;
    let gpxy_min = e.0.px.y;
    let gpxy_max = gpxy_min + e.0.height;
    cursor_position_res.0.x > gpxx_min as f32 && cursor_position_res.0.x < gpxx_max as f32
      && cursor_position_res.0.y > gpxy_min as f32 && cursor_position_res.0.y < gpxy_max as f32
  }).map(|e| e.1) {
    let entrance = entrance_positional_query.get(entrance.0).expect("Entrance is required at this point");
    println!("tile to pathfind: {:?}", entrance.0);
  }
}

#[derive(Resource, Deref, DerefMut)]
pub struct CursorPos(pub Vec2);
impl Default for CursorPos {
  fn default() -> Self {
    // Initialize the cursor pos at some far away place. It will get updated
    // correctly when the cursor moves.
    Self(Vec2::new(-1000.0, -1000.0))
  }
}

pub fn update_cursor_pos(
  camera_q: Query<(&GlobalTransform, &Camera)>,
  mut cursor_moved_events: EventReader<CursorMoved>,
  mut cursor_pos: ResMut<CursorPos>,
  level_height: Res<LevelHeight>
) {
  for cursor_moved in cursor_moved_events.iter() {
    for (cam_t, cam) in camera_q.iter() {
      if let Some(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
        **cursor_pos = Vec2::new(pos.x, (*level_height).0 as f32 - pos.y);
      }
    }
  }
}

// fn ls2 (q: Query<&UrlPath>) {
//   for e in q.iter() {
//     println!("path.. {}", e.0);
//   }
// }



fn setup(mut commands: Commands) {
  // ui camera
  commands.spawn(Camera2dBundle::default());

}

fn setup_ldtk(mut commands: Commands, asset_server: Res<AssetServer>) {
  let bundle = LdtkWorldBundle {
    ldtk_handle: asset_server.load("ldtk/town.ldtk"),
    ..Default::default()
  };
  commands.spawn(bundle);
}

#[derive(Debug, Default, Component)]
pub struct LevelPositional {
  pub px: IVec2,
  pub width: i32,
  pub height: i32,
}

impl LevelPositional {
  pub fn from_entity_field(entity_instance: &EntityInstance) -> LevelPositional {
    return LevelPositional {
      px: entity_instance.px,
      width: entity_instance.width,
      height: entity_instance.height,
    }
  }
}

#[derive(Debug, Default, Component)]
pub struct BuildingArea;

#[derive(Bundle, LdtkEntity)]
pub struct BuildingAreaBundle {
  #[with(UnresolvedBuildingEntranceRef::from_building_entrance_field)]
  unresolved_building_entrance: UnresolvedBuildingEntranceRef,
  #[with(LevelPositional::from_entity_field)]
  positional: LevelPositional,
  #[grid_coords]
  grid_coords: GridCoords,
  building_area: BuildingArea,
  // #[sprite_sheet_bundle]
  // #[bundle]
  // sprite_bundle: SpriteSheetBundle,
}

#[derive(Debug, Default, Component, Reflect)]
pub struct UrlPath(String);

impl UrlPath {
  pub fn from_field(entity_instance: &EntityInstance) -> UrlPath {
    UrlPath(
      entity_instance
        .get_string_field("path")
        .expect("expected entity to have non-nullable path str field").to_string(),
    )
  }
}

#[derive(Debug, Default, Component)]
pub struct BuildingEntrance;

#[derive(Bundle, LdtkEntity)]
pub struct BuildingEntranceBundle {
  #[with(UrlPath::from_field)]
  path: UrlPath,
  #[with(LevelPositional::from_entity_field)]
  positional: LevelPositional,
  #[grid_coords]
  grid_coords: GridCoords,
  building_entrance: BuildingEntrance,
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_interaction_system(
  mut interaction_query: Query<
    (
      &Interaction,
      &mut BackgroundColor,
      &Children,
    ),
    (Changed<Interaction>, With<Button>),
  >,
  mut text_query: Query<&mut Text>,
) {
  for (interaction, mut color, children) in &mut interaction_query {
    let mut text = text_query.get_mut(children[0]).unwrap();
    match *interaction {
      Interaction::Clicked => {
        text.sections[0].value = "Press".to_string();
        *color = PRESSED_BUTTON.into();
        go_town("test");
      }
      Interaction::Hovered => {
        text.sections[0].value = "Hover".to_string();
        *color = HOVERED_BUTTON.into();
      }
      Interaction::None => {
        text.sections[0].value = "Button".to_string();
        *color = NORMAL_BUTTON.into();
      }
    }
  }
}

fn button_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands
    .spawn(NodeBundle {
      style: Style {
        size: Size {
          width: Val::Percent(100.0),
          height: Default::default(),
        },
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
      },
      ..default()
    })
    .with_children(|parent| {
      parent
        .spawn(ButtonBundle {
          style: Style {
            size: Size {
              width: Val::Px(150.0),
              height: Val::Px(65.0),
            },

            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
          },
          background_color: NORMAL_BUTTON.into(),
          ..default()
        })
        .with_children(|parent| {
          parent.spawn(TextBundle::from_section(
            "Button",
            TextStyle {
              font: asset_server.load("fonts/FiraSans-Bold.ttf"),
              font_size: 40.0,
              color: Color::rgb(0.9, 0.9, 0.9),
            },
          ));
        });
    });
}
