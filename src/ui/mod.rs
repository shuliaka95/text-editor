mod window;
mod widgets;

use anyhow::Result;
use egui::Context;

pub use window::App;

pub struct UI {
    ctx: Context,
    app: App,
}

impl UI {
    pub fn new() -> Result<Self> {
        Ok(Self {
            ctx: Context::default(),
            app: App::new()?,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        todo!()
    }
}



