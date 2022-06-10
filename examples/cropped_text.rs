use embedded_canvas::Canvas;
use embedded_graphics::{
    mono_font::{ascii::FONT_9X18_BOLD, MonoTextStyle},
    pixelcolor::Rgb555,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

pub const DISPLAY_90P: Size = Size::new(150, 90);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We draw:
    //   1. A box with an outline
    //   2. The box should contain 3 lines of text
    //   3. Only the middle line will be fully shown while
    //      the others will be partially visible

    let mut display = SimulatorDisplay::<Rgb555>::new(DISPLAY_90P);

    let canvas_size = Size {
        width: 100,
        height: 46,
    };

    let mut canvas = {
        let mut canvas: Canvas<Rgb555> = Canvas::new(canvas_size);

        let canvas_box = Rectangle::new(Point::zero(), canvas_size);

        let canvas_style = canvas_box.into_styled(PrimitiveStyle::with_stroke(Rgb555::YELLOW, 4));

        canvas_style.draw(&mut canvas)?;

        canvas
    };

    let text_str = "Cropping a \ntext into a \nmagic box.";
    let style = MonoTextStyle::new(&FONT_9X18_BOLD, Rgb555::WHITE);
    let text = Text::new(text_str, Point::new(4, 8), style);

    text.draw(&mut canvas)?;

    let crop_area = Rectangle::with_center(canvas.center(), canvas_size);

    let cropped_canvas = canvas
        .crop(&crop_area)
        .expect("Should crop")
        .place_at(Point::new(150, 120) / 6);

    cropped_canvas.draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    Window::new("Canvas with cropping", &output_settings).show_static(&display);

    Ok(())
}
