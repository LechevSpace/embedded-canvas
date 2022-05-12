extern crate alloc;

use alloc::{boxed::Box, vec};


use embedded_graphics::{prelude::*, primitives::Rectangle};

pub struct Canvas<C: PixelColor> {
    pub top_left: Point,
    pub canvas: Size,
    pub(crate) pixels: Box<[Option<C>]>,
}

impl<C: PixelColor> Canvas<C> {
    pub fn new(top_left: Point, canvas: Size) -> Self {
        let pixel_count = canvas.width as usize * canvas.height as usize;

        let pixels = vec![None; pixel_count].into_boxed_slice();

        Self {
            top_left,
            canvas,
            pixels,
        }
    }

    pub fn with_default_color(top_left: Point, canvas: Size, default_color: C) -> Self {
        let pixel_count = canvas.width as usize * canvas.height as usize;

        let pixels = vec![Some(default_color); pixel_count].into_boxed_slice();

        Self {
            top_left,
            canvas,
            pixels,
        }
    }

    pub fn with_center(center: Point, size: Size) -> Self {
        // todo: use our own center_offset like in this assoc. fn
        let rect = Rectangle::with_center(center, size);

        Self::new(rect.top_left, size)
    }

    /// Returns the center of the bounding box
    pub fn center(&self) -> Point {
        self.bounding_box().center()
    }

    /// Returns the color of the pixel at a point.
    ///
    /// Returns `None` if `point` is outside the display.
    pub fn get_pixel(&self, point: Point) -> Option<C> {
        self.point_to_index(point)
            .and_then(|index| self.pixels.get(index).copied().flatten())
    }

    // TODO: make safer
    fn point_to_index(&self, point: Point) -> Option<usize> {
        // we must account for the top-left corner of the drawing box
        if let Ok((x, y)) = <(u32, u32)>::try_from(point - self.top_left) {
            if x < self.canvas.width && y < self.canvas.height {
                return Some((x + y * self.canvas.width) as usize);
            }
        }

        None
    }

    // TODO: make safer
    fn index_to_point(&self, index: usize) -> Option<Point> {
        let x = index as i32 % self.canvas.width as i32;
        let y = index as i32 / self.canvas.width as i32;
        let point = Point { x, y };

        Some(point)
    }

    pub fn crop(&self, area: &Rectangle) -> Option<Canvas<C>>
    {
        let mut new = Canvas::new(area.top_left, area.size);

        let new_pixels = self
            .pixels
            .into_iter()
            .enumerate()
            .filter_map(|(index, color)| {
                let color = match color {
                    Some(color) => *color,
                    None => return None,
                };

                // we account for the displacement of the current Canvas
                let point = self.index_to_point(index).unwrap() + self.top_left;

                // for here on, we should compare the point based on the area we want to crop
                if point >= area.top_left && point <= area.bottom_right().unwrap() {
                    let pixel = Pixel(point, color);

                    Some(pixel)
                } else {
                    None
                }
            });

        new.draw_iter(new_pixels).ok().map(|_| { 
            // keep the original top_left point of the Canvas
            new.top_left = self.top_left;
            new
        })
    }
}

impl<C: PixelColor> Dimensions for Canvas<C> {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.top_left, self.canvas)
    }
}

trait DrawCanvas {}

impl<C: PixelColor> DrawTarget for Canvas<C> {
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            if let Some(index) = self.point_to_index(point) {
                self.pixels[index] = Some(color);
            }
        }

        Ok(())
    }
}

impl<C> Drawable for Canvas<C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let pixels_iter = self.bounding_box().points().filter_map(|point| {
            let color = self.get_pixel(point);

            match color {
                // for the drawing position we need to account for the top_left offset of the drawn display
                Some(color) => Some(Pixel(point /* + self.top_left */, color)),
                None => None,
            }
            // if color.is_some() {
            //     println!("Pixel at: {:?} has COLOR", point);
            // }

            // let color = color?;

            // Some(Pixel(point + self.top_left, color))
        });

        target.draw_iter(pixels_iter)
    }
}

impl<C: PixelColor> Transform for Canvas<C> {
    fn translate(&self, by: Point) -> Self {
        Self {
            top_left: self.top_left + by,
            canvas: self.canvas,
            pixels: self.pixels.clone(),
        }
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.top_left += by;

        self
    }
}

#[cfg(test)]
mod test {
    use embedded_graphics::pixelcolor::BinaryColor;

    use super::*;

    #[test]
    fn test_index_to_point() {
        let canvas = Canvas::<BinaryColor>::new(
            Point::new(0, 0),
            Size {
                width: 320,
                height: 240,
            }
        );

        {
            let center = Point::new(160, 120);
            let center_index = canvas.point_to_index(center).expect("Inside the canvas");

            assert_eq!(
                center,
                canvas
                    .index_to_point(center_index)
                    .expect("Should fetch the index")
            );
        }
        {
            let bottom_right = Point::new(320 - 1, 240 - 1);
            let br_index = canvas
                .point_to_index(bottom_right)
                .expect("Inside the canvas");

            assert_eq!(
                bottom_right,
                canvas
                    .index_to_point(br_index)
                    .expect("Should fetch the index")
            );
        }
        {
            let top_left = Point::new(0, 0);
            let tl_index = canvas.point_to_index(top_left).expect("Inside the canvas");

            assert_eq!(
                top_left,
                canvas
                    .index_to_point(tl_index)
                    .expect("Should fetch the index")
            );
        }

        {
            let bottom_left = Point::new(0, 240 - 1);
            let bl_index = canvas
                .point_to_index(bottom_left)
                .expect("Inside the canvas");

            assert_eq!(
                bottom_left,
                canvas
                    .index_to_point(bl_index)
                    .expect("Should fetch the index")
            );
        }
        {
            let top_right = Point::new(320 - 1, 0);
            let tr_index = canvas.point_to_index(top_right).expect("Inside the canvas");

            assert_eq!(
                top_right,
                canvas
                    .index_to_point(tr_index)
                    .expect("Should fetch the index")
            );
        }
    }
}
