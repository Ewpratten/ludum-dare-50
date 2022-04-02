use nalgebra as na;


#[derive(Debug, Clone)]
pub struct Player {
    pub position: na::Vector2<f32>,
    pub velocity: na::Vector2<f32>,
    pub size: f32,
}

impl Player {
    
    /// Construct a new player.
    pub fn new(position: na::Vector2<f32>) -> Self {
        Self {
            position,
            velocity: na::Vector2::zeros(),
            size: 1.0, 
        }
    }

}
