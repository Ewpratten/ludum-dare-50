use nalgebra as na;

/// Converts from the tiled coordinate system to the game coordinate system.
pub fn tiled_to_game(vec: na::Vector2<f32>) -> na::Vector2<f32> {
    na::Vector2::new(vec.x, vec.y * -1.0)
}

/// Converts from the game coordinate system to the tiled coordinate system.
pub fn game_to_tiled(vec: na::Vector2<f32>) -> na::Vector2<f32> {
    tiled_to_game(vec)
}