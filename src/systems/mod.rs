mod collisions;
mod entity_render;
mod map_renderer;
mod player_input;
mod random_move;
use crate::{end_turn, prelude::*};

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_renderer::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}
pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_renderer::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_renderer::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
