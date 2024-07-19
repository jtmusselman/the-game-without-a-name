// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use the_game_without_a_name::AppPlugin;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}
