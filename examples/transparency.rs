use embedded_canvas::Canvas;
use embedded_graphics::{
    pixelcolor::Rgb555,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle, StyledDrawable},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

pub const DISPLAY_360P: Size = Size::new(480, 360);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // In this example we:
    // 1. Draw a square (200 x 200) in the middle of the display
    // 2. Create a Canvas (100 x 100)
    // 3. Draw a circle (diameter 99) in the center of the canvas
    // 4. Draw the canvas on the middle of the display

    let mut display = SimulatorDisplay::<Rgb555>::new(DISPLAY_360P);

    let rectangle_size = Size {
        width: 200,
        height: 200,
    };

    let canvas_size = Size {
        width: 100,
        height: 100,
    };

    let rectangle_fill = PrimitiveStyle::with_fill(Rgb555::BLUE);
    let circle_fill = PrimitiveStyle::with_fill(Rgb555::YELLOW);

    // Created a rectangle filled with color and drawn on display.
    Rectangle::with_center(display.bounding_box().center(), rectangle_size)
        .draw_styled(&rectangle_fill, &mut display)?;

    // Draw a circle filled with color on Canvas
    let canvas = {
        let mut canvas: Canvas<Rgb555> = Canvas::new(canvas_size);

        // Draw a circle on the canvas and fill with color
        Circle::with_center(canvas.bounding_box().center(), 99)
            .draw_styled(&circle_fill, &mut canvas)?;

        canvas
    };

    // place the canvas in the center of the display
    canvas
        .place_center(display.bounding_box().center())
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().build();

    Window::new("Transparency", &output_settings).show_static(&display);

    Ok(())
}
