mod button;
mod timer_transition;

use bevy::prelude::*;

use bevy_mod_picking::prelude::*;
use button::{main_menu_interaction, MainMenuButtonWidget, MainMenuButtonWidgetBundle};
use woodpecker_ui::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_widget::<MainMenuButtonWidget>()
            .add_systems(Startup, spawn_main_menu)
            .add_systems(
                Update,
                (timer_transition::update_transitions, main_menu_interaction),
            );
    }
}

fn spawn_main_menu(mut commands: Commands, mut ui_context: ResMut<WoodpeckerContext>) {
    commands.spawn(Camera2dBundle::default());

    let mut buttons = WidgetChildren::default();
    buttons.add::<MainMenuButtonWidget>((
        MainMenuButtonWidgetBundle {
            props: MainMenuButtonWidget {
                content: "New Game".to_string(),
                ..default()
            },
            ..default()
        },
        On::<Pointer<Click>>::run(|| {
            info!("clicked");
        }),
    ));
    buttons.add::<MainMenuButtonWidget>((
        MainMenuButtonWidgetBundle {
            props: MainMenuButtonWidget {
                content: "Options".to_string(),
                ..default()
            },
            ..default()
        },
        On::<Pointer<Click>>::run(|| {
            info!("clicked");
        }),
    ));
    buttons.add::<MainMenuButtonWidget>((
        MainMenuButtonWidgetBundle {
            props: MainMenuButtonWidget {
                content: "Exit".to_string(),
                ..default()
            },
            ..default()
        },
        On::<Pointer<Click>>::run(|mut exit: EventWriter<AppExit>| {
            exit.send(AppExit::Success);
        }),
    ));

    let root = commands
        .spawn((WoodpeckerAppBundle {
            children: WidgetChildren::default().with_child::<Element>(ElementBundle {
                styles: WoodpeckerStyle {
                    width: Units::Percentage(100.0),
                    height: Units::Percentage(100.0),
                    justify_content: Some(WidgetAlignContent::FlexStart),
                    align_content: Some(WidgetAlignContent::Center),
                    padding: Edge {
                        left: 0.0.into(),
                        right: 0.0.into(),
                        top: 25.0.into(),
                        bottom: 0.0.into(),
                    },
                    ..Default::default()
                },

                children: WidgetChildren::default().with_child::<Element>((ElementBundle {
                    styles: WoodpeckerStyle {
                        background_color: Srgba::hex("FF007F").unwrap().into(),
                        margin: Edge {
                            top: Units::Pixels(25.),
                            right: Units::Pixels(25.),
                            bottom: Units::Pixels(25.),
                            left: Units::Pixels(25.),
                        },
                        width: Units::Pixels(300.),
                        height: Units::Pixels(300.),
                        gap: (Units::Pixels(10.), Units::Pixels(5.)),
                        justify_content: Some(WidgetAlignContent::Center),
                        align_content: Some(WidgetAlignContent::Center),
                        display: WidgetDisplay::Flex,
                        flex_direction: WidgetFlexDirection::Column,
                        position: WidgetPosition::Relative,
                        ..Default::default()
                    },
                    children: buttons,
                    ..Default::default()
                },)),
                ..Default::default()
            }),
            ..Default::default()
        },))
        .id();

    ui_context.set_root_widget(root);
}
