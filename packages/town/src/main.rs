use wasm_bindgen::prelude::*;

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

use bevy::{prelude::*, winit::WinitSettings};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(WinitSettings::game())
    .add_startup_system(setup)
    .add_system(button_system)
    .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  // ui camera
  commands.spawn(Camera2dBundle::default());
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


