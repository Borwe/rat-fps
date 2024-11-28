# RAT-FPS

### Description:
A Widget for use with Ratatui to display FPS

## Example Usage:

```rust
use std::{error::Error, time::Duration};

use rat_fps::FPS;
use ratatui::crossterm::{self, event::{Event, KeyCode, KeyEvent}};

fn main() -> Result<(), Box<dyn Error>> {
    let mut term = ratatui::init();
    let mut keep_running = true;
    let mut fps = FPS::new()?;

    while keep_running {
        term.draw(|f|{
            //render the fps
            f.render_widget(&mut fps, f.area());
        })?;

        //wait roughly to enable rpughly atleast 60fps if possible, 
        // note what you do after the poll might reduce fps if not fast enough,
        // in such cases it is adviced to have a higher wait for fps,
        //roughly 30+ above expected
        if crossterm::event::poll(Duration::from_millis(fps.wait_for_fps(60)?))? {
            if let Event::Key(KeyEvent{ 
                code: KeyCode::Char('q'), ..}) = crossterm::event::read()? {
                keep_running = false;
            }
        }
    }

    ratatui::restore();
    Ok(())
}
```
