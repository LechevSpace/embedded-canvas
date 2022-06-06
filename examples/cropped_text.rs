use embedded_canvas::Canvas;
use embedded_graphics::{
    pixelcolor::{Rgb555},
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

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

    let mut display = SimulatorDisplay::<Rgb555>::new(DISPLAY_240P);

    let canvas_size = Size {
        width: DISPLAY_360P.width / 2,
        height: DISPLAY_360P.height / 2
    };

    let mut canvas = {
        let mut canvas: Canvas<Rgb555> = Canvas::new(canvas_size);

        let canvas_box = Rectangle::new(Point::zero(), canvas_size);

        let canvas_style = canvas_box.into_styled(PrimitiveStyle::with_stroke(Rgb555::GREEN, 4));

        canvas_style.draw(&mut canvas)?;

        canvas
    };

    canvas.place_at(canvas.center() / 3).draw(&mut display)?;

    // let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

    // let bottom_right = Size::zero() + display.size() - Size::new(0, 0);

    // Rectangle::new(Point::zero(), bottom_right)
    //     .into_styled(thick_stroke)
    //     .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    Window::new("Canvas with cropping", &output_settings).show_static(&display);

    Ok(())
}
