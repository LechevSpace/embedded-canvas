use embedded_canvas::Canvas;
use embedded_graphics::{
    mono_font::{
        ascii::{FONT_10X20, FONT_9X18_BOLD},
        MonoTextStyle,
    },
    pixelcolor::Rgb555,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

pub const DISPLAY_240P: Size = Size::new(320, 240);
pub const DISPLAY_360P: Size = Size::new(480, 360);
pub const DISPLAY_720P: Size = Size::new(1280, 720);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We draw:
    //   1. A box with an outline
    //   2. The box should contain 3 lines of text
    //   3. Only the middle line will be fully shown while
    //      the others will be partially visible

    let mut display = SimulatorDisplay::<Rgb555>::new(DISPLAY_360P);

    let canvas_size = Size {
        width: 200,
        height: 57,
    };

    let mut canvas: Canvas<Rgb555> = Canvas::new(canvas_size);

    let canvas_outline = Rectangle::new(Point::zero(), canvas_size);
    let canvas_style = PrimitiveStyle::new();
    let canvas_rectangle = canvas_outline.into_styled(canvas_style);

    canvas_rectangle.draw(&mut canvas)?;

    let text_str = "Cropping a \ntext into a \nmagic box.";
    let style = MonoTextStyle::new(&FONT_10X20, Rgb555::WHITE);
    let text = Text::new(text_str, Point::new(4, 15), style);

    text.draw(&mut canvas)?;

    let crop_area = Rectangle::new(
        Point::zero(),
        Size {
            width: 120,
            height: 45,
        },
    );

    let cropped_canvas = canvas
        .crop(&crop_area)
        .expect("Should crop")
        .place_at(Point { x: 50, y: 50 });

    cropped_canvas.draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        // .theme(BinaryColorTheme::OledBlue)
        .build();

    Window::new("Canvas with cropping", &output_settings).show_static(&display);

    Ok(())
}
