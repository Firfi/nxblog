mod building_entrance_ref;

use wasm_bindgen::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use building_entrance_ref::*;

use bevy::{prelude::*, winit::WinitSettings};

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
    // .add_startup_system(button_setup)
    // .add_system(button_interaction_system)
    .insert_resource(LevelSelection::Index(0))
    .register_ldtk_entity::<BuildingAreaBundle>("BuildingArea")
    .register_ldtk_entity::<BuildingEntranceBundle>("BuildingEntrance")
    .add_system(ls)
    .add_system(ls2)
    .register_type::<UrlPath>()
    .register_type::<BuildingEntranceRef>()
    .run();
}

fn ls (q: Query<&BuildingEntranceRef>) {
  for e in q.iter() {
    println!("path.. {:?}", e);
  }
}

fn ls2 (q: Query<&UrlPath>) {
  for e in q.iter() {
    println!("path.. {}", e.0);
  }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);



fn setup(mut commands: Commands) {
  // ui camera
  commands.spawn(Camera2dBundle::default());

}

fn setup_ldtk(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(LdtkWorldBundle {
    ldtk_handle: asset_server.load("ldtk/town.ldtk"),
    ..Default::default()
  });
}

#[derive(Default, Component)]
pub struct BuildingArea;

#[derive(Bundle, LdtkEntity)]
pub struct BuildingAreaBundle {
  #[with(UnresolvedBuildingEntranceRef::from_building_entrance_field)]
  unresolved_building_entrance: UnresolvedBuildingEntranceRef,
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


#[derive(Bundle, LdtkEntity)]
pub struct BuildingEntranceBundle {
  #[with(UrlPath::from_field)]
  path: UrlPath,
}

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
