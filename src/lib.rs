use std::{error::Error, fmt::Write, time::{SystemTime, UNIX_EPOCH}};

use ratatui::{style::Stylize, widgets::{Block, Paragraph, Widget}};

/// Note: This implements the widget using the mutable reference,
/// so when rendering one must pass a mut reference to this object
pub struct FPS {
    prev_time: u128,
    fps_text: String
}

impl FPS {
    /// Create new widget
    pub fn new() -> Result<Self, Box<dyn Error>>{
        Ok(Self{
            prev_time: FPS::now_millis()?,
            fps_text: String::with_capacity(1024)
        })
    }

    /// get time to wait for in poll for example with crosster::event::poll,
    /// it is in milliseconds, calculated from using the @expected_fps
    /// field value
    pub fn wait_for_fps(&self, expected_fps: u128) -> Result<u64, Box<dyn Error>> {
        let now = Self::now_millis()?;
        let dif = now - self.prev_time;
        if expected_fps>0 && dif > 1000/expected_fps {
            Ok(0)
        }else{
            Ok(1000/expected_fps as u64)
        }
    }

    fn now_millis ()-> Result<u128, Box<dyn Error>> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
        Ok(now.as_millis())
    }

    fn set_fps(&mut self) -> Result<(), Box<dyn Error>> {
        let now = FPS::now_millis()?;
        let time_passed = now - self.prev_time;
        self.prev_time = now;
        let mut fps = 1;
        if time_passed >0 {
            fps = 1000/time_passed;
        }
        self.fps_text.clear();
        write!(&mut self.fps_text, "{} FPS", fps)?;
        Ok(())
    }
}

/// Implements Widget on mutable reference
impl Widget for &mut FPS {
    fn render(self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer) where Self: Sized {
        self.set_fps().unwrap();
        let text = Paragraph::new(self.fps_text.as_str()).bold()
            .green();

        let mut new_area = area.clone();
        if area.height >= 3 {
            let len_of_str = self.fps_text.len()+2;
            if (len_of_str as u16) < area.width {
                new_area.width = len_of_str as u16
            }
            new_area.height = 3;
            text.block(Block::bordered()).render(new_area,buf);
            return;
        }

        new_area.width = self.fps_text.len() as u16;
        text.render(area,buf);
    }
}
