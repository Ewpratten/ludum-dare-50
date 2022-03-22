use raylib::prelude::*;

use crate::discord::DiscordChannel;

#[derive(Debug)]
pub struct SmFailureScreen {}

impl SmFailureScreen {
    /// Construct a new `SmFailureScreen`
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(
        &mut self,
        raylib: &mut RaylibHandle,
        rl_thread: &RaylibThread,
        discord: &DiscordChannel,
    ) -> bool {
        let mut d = raylib.begin_drawing(&rl_thread);

        d.clear_background(raylib::color::Color::RED);
        d.draw_text("Backend Rendering Broke.\nYou should not be seeing this!", 10, 10, 40, raylib::color::Color::WHITE);

        false
    }
}
