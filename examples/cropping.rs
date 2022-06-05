use embedded_canvas::Canvas;
use embedded_graphics::{prelude::*, pixelcolor::Rgb555};
use embedded_graphics_simulator::SimulatorDisplay;

pub const DISPLAY_240P: Size = Size::new(320, 240);
pub const DISPLAY_360P: Size = Size::new(480, 360);
pub const DISPLAY_720P: Size = Size::new(1280, 720);

fn main() -> Result<(), Box<dyn std::error::Error>> {

    /* Algorithms:
       1. Show a box with an outline
       2. The box should contain 3 lines of text
       3. Only the middle line will be fully shown while 
          the others will be partially visible 
    */

    let mut display = SimulatorDisplay::<Rgb555>::new(DISPLAY_360P);

    Ok(())
}