use embedded_canvas::Canvas;
use embedded_graphics::{
    pixelcolor::Rgb555,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

pub const DISPLAY_240P: Size = Size::new(320, 240);
pub const DISPLAY_360P: Size = Size::new(480, 360);
pub const DISPLAY_720P: Size = Size::new(1280, 720);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut display = SimulatorDisplay::<Rgb555>::new(DISPLAY_360P);

    let rec_size = Size {
        width: 200,
        height: 200,
    };

    let canvas_size = Size {
        width: 100,
        height: 100,
    };

    let rectangle = Rectangle::new(Point::new(140, 75), rec_size)
        .into_styled(PrimitiveStyle::with_fill(Rgb555::BLUE));
    rectangle.draw(&mut display)?;

    let canvas = {
        let mut canvas: Canvas<Rgb555> = Canvas::new(canvas_size);

        let circle =
            Circle::new(Point::zero(), 99).into_styled(PrimitiveStyle::with_fill(Rgb555::BLACK));
        circle.draw(&mut canvas)?;

        canvas
    };

    canvas.place_at(Point::new(190, 120)).draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().build();

    Window::new("Transparency", &output_settings).show_static(&display);

    Ok(())
}
