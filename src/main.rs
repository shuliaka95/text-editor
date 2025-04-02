mod editor;
mod ui;
mod utils;

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    
    let mut app = ui::App::new()?;
    app.run()?;

    Ok(())
}
