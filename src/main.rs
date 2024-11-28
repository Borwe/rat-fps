
use std::{error::Error, time::Duration};

use rat_fps::FPS;
use ratatui::crossterm::{self, event::{Event, KeyCode, KeyEvent}};

fn main() -> Result<(), Box<dyn Error>> {
    let mut term = ratatui::init();
    let mut keep_running = true;
    let mut fps = FPS::new()?;

    while keep_running {
        term.draw(|f|{
            f.render_widget(&mut fps, f.area());
        })?;

        if crossterm::event::poll(Duration::from_millis(fps.wait_for_fps(1000)?))? {
            if let Event::Key(KeyEvent{ 
                code: KeyCode::Char('q'), ..}) = crossterm::event::read()? {
                keep_running = false;
            }
        }
    }

    ratatui::restore();
    Ok(())
}
