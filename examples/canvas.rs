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

    // 1. Draw the Canvas with an outline rectangle (on the left)
    //
    // 2. Draw on the center of the canvas and crop it (on the top right):
    // - a rectangle - 100 x 100
    // - circle - 98 diameter (1 pixel on each side accounts for the rectangle size)
    //
    // 3. Take the second canvas and crop the rectangle. Draw it below 2. (on the bottom right)
    {
        let canvas_size = Size {
            width: 200,
            height: 200,
        };

        // first canvas - not cropped and on the left side
        // it has an outline to see that it's actually cropped
        let mut canvas = {
            let mut canvas = Canvas::<Rgb555>::new(canvas_size);

            // draw a rectangle smaller than the canvas (with 1px)
            let canvas_rectangle = Rectangle::new(Point::zero(), canvas_size);

            let canvas_outline =
                canvas_rectangle.into_styled(PrimitiveStyle::with_stroke(Rgb555::YELLOW, 1));
            // draw the canvas rectangle for debugging
            canvas_outline.draw(&mut canvas)?;

            canvas
        };

        // Place the canvas at a specific point on the display and draw it
        canvas.place_at(Point::zero()).draw(&mut display)?;

        // prepare for drawing
        let drawing_size = Size {
            width: 100,
            height: 100,
        };

        // create a rectangle as big as the drawing inside the canvas
        let rectangle = Rectangle::with_center(canvas.center(), drawing_size)
            .into_styled(PrimitiveStyle::with_stroke(Rgb555::WHITE, 1));
        rectangle.draw(&mut canvas)?;

        // create a circle less than the rectangle of the drawing
        let circle = Circle::with_center(canvas.center(), 98)
            .into_styled(PrimitiveStyle::with_stroke(Rgb555::RED, 1));
        circle.draw(&mut canvas)?;

        // prepare the are to be cropped
        let crop_area = Rectangle::with_center(canvas.center(), drawing_size);

        // crop the canvas
        let cropped_canvas = canvas
            .crop(&crop_area)
            .expect("Should crop")
            // place the cropped canvas 80px away from the original canvas with the outline
            .place_at(Point {
                x: canvas.bounding_box().size.width as i32 + 80,
                y: 0,
            });

        // draw the cropped Canvas on to the display and offset it by 80px to the right of the non-cropped one
        cropped_canvas.draw(&mut display)?;

        // Crop from the cropped canvas, only the circle
        // remove the stroke of the drawn rectangle
        let crop_rectangle =
            Rectangle::with_center(cropped_canvas.center(), drawing_size - Size::new(2, 2));
        cropped_canvas
            .crop(&crop_rectangle)
            .expect("Should crop")
            .translate(Point::new(0, 100))
            .draw(&mut display)?;
    }

    let output_settings = OutputSettingsBuilder::new().build();

    Window::new(
        "Canvas with outline (left) & 2 CanvasAt (right)",
        &output_settings,
    )
    .show_static(&display);

    Ok(())
}
