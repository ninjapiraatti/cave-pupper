pub mod bindings;

pub use bindings::{InputBindings, PlayerInputs, MAX_SLOTS};

use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputBindings>()
            .init_resource::<PlayerInputs>()
            .add_systems(PreUpdate, read_player_inputs);
    }
}

fn read_player_inputs(
    keyboard: Res<ButtonInput<KeyCode>>,
    bindings: Res<InputBindings>,
    mut inputs: ResMut<PlayerInputs>,
    time: Res<Time>,
) {
    inputs.update(&keyboard, &bindings, time.elapsed_secs());
}
