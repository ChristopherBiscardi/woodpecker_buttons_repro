use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use woodpecker_buttons::MainMenuPlugin;
use woodpecker_ui::WoodpeckerUIPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DefaultPickingPlugins,
            WoodpeckerUIPlugin,
            MainMenuPlugin,
        ))
        .run();
}
