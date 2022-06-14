use embedded_canvas::Canvas;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb555,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle, StyledDrawable},
    text::Text,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

pub const DISPLAY_240P: Size = Size::new(320, 240);
pub const DISPLAY_360P: Size = Size::new(480, 360);
pub const DISPLAY_720P: Size = Size::new(1280, 720);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // In this example we:
    // 1. Create and draw an entire canvas on the display (on the left)
    //   - Filled the canvas with the color RED.
    //   - Write a text saying "This text will be\ncropped here"
    // 2. Crop the text "cropped here" from the entire canvas (on the right)

    let mut display = SimulatorDisplay::<Rgb555>::new(DISPLAY_360P);

    let canvas_size = Size {
        width: 180,
        height: 45,
    };

    // Draw on a Canvas filled with color and write text
    let entire_canvas = {
        let mut canvas: Canvas<Rgb555> = Canvas::new(canvas_size);

        let canvas_fill = PrimitiveStyleBuilder::new().fill_color(Rgb555::RED).build();

        // Fill the canvas with a color to be more visible what we actually crop
        Rectangle::new(Point::zero(), canvas_size).draw_styled(&canvas_fill, &mut canvas)?;

        let text_str = "This text will be\ncropped here";
        let style = MonoTextStyle::new(&FONT_10X20, Rgb555::WHITE);
        let text = Text::new(text_str, Point::new(4, 15), style);

        text.draw(&mut canvas)?;

        canvas
    };

    // place the canvas on the display
    entire_canvas
        .place_at(Point::new(5, 10))
        .draw(&mut display)
        .expect("Should draw canvas");

    // Crop only the text saying "smaller box" from the entire_canvas
    let cropped_canvas = {
        let crop_area = Rectangle::new(
            Point::new(0, 20),
            Size {
                width: 130,
                height: 30,
            },
        );

        entire_canvas.crop(&crop_area).expect("Should crop")
    };

    // Place the cropped canvas on the right side of the entire_canvas
    cropped_canvas
        .place_at(Point { x: 220, y: 10 })
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().build();

    Window::new("Canvas with cropping", &output_settings).show_static(&display);

    Ok(())
}
