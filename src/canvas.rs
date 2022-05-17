extern crate alloc;

use alloc::{boxed::Box, vec};

use embedded_graphics_core::{prelude::*, primitives::Rectangle};

/// Canvas on which you can draw but it's not drawable on the display yet.
///
/// Draw on the [`Canvas`] using origin of [`Point::zero()`].
pub struct Canvas<C: PixelColor> {
    /// The size of the [`Canvas`].
    pub canvas: Size,
    /// The pixels of the [`Canvas`].
    pub pixels: Box<[Option<C>]>,
}

impl<C: PixelColor> Canvas<C> {
    /// Create a new blank [`Canvas`].
    pub fn new(canvas: Size) -> Self {
        Self {
            canvas,
            pixels: new_pixels(canvas, None),
        }
    }

    /// Create a [`Canvas`] filled with a default color.
    pub fn with_default_color(canvas: Size, default_color: C) -> Self {
        Self {
            canvas,
            pixels: new_pixels(canvas, default_color.into()),
        }
    }

    /// Helper method that returns the index in the array of pixels
    fn point_to_index(&self, point: Point) -> Option<usize> {
        point_to_index(self.canvas, Point::zero(), point)
    }

    fn index_to_point(&self, index: usize) -> Option<Point> {
        index_to_point(self.canvas, index)
    }

    /// Returns the center of [`Size`] of the [`Canvas`].
    pub fn center(&self) -> Point {
        Point::zero() + center_offset(self.canvas)
    }

    /// Create a new cropped [`Canvas`].
    ///
    /// This method takes into account the top left [`Point`] of the `area`
    /// you'd like to crop relative to the [`Canvas`] itself.
    //
    /// If the width or height of the [`Rectangle`] is `0`, this method will
    /// return [`None`] (see [`Rectangle::bottom_right()`])
    // todo: make safer
    pub fn crop(&self, area: &Rectangle) -> Option<Canvas<C>> {
        let mut new = Canvas::new(area.size);

        // returns None when width or height is `0`
        // it's safe to return `None` for Canvas too!
        let area_bottom_right = area.bottom_right()?;

        let new_pixels = self.pixels.iter().enumerate().filter_map(|(index, color)| {
            let color = match color {
                Some(color) => *color,
                None => return None,
            };

            // Canvas always starts from `Point::zero()`
            let point = self.index_to_point(index).unwrap();

            // for here on, we should compare the point based on the area we want to crop
            if point >= area.top_left && point <= area_bottom_right {
                // remove the area top_left to make the origin at `Point::zero()` for the cropped part
                let pixel = Pixel(point - area.top_left, color);

                Some(pixel)
            } else {
                None
            }
        });

        new.draw_iter(new_pixels).ok().map(|_| new)
    }

    /// Sets the place with top left offset where the canvas will be drawn to the display.
    pub fn place_at(&self, top_left: Point) -> CanvasAt<C> {
        CanvasAt {
            top_left,
            canvas: self.canvas,
            pixels: self.pixels.clone(),
        }
    }

    /// Sets the center of the [`Canvas`] where it will be drawn to the display.
    pub fn place_center(&self, center: Point) -> CanvasAt<C> {
        let top_left = center - center_offset(self.canvas);

        self.place_at(top_left)
    }
}

impl<C: PixelColor> OriginDimensions for Canvas<C> {
    fn size(&self) -> Size {
        self.canvas
    }
}

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

/// Canvas which is drawable at the provided [`Point`] (location) on the display.
#[derive(Debug, Clone)]
pub struct CanvasAt<C: PixelColor> {
    /// The top left offset where the [`CanvasAt`] will be drawn to the display.
    pub top_left: Point,
    /// The size of the [`CanvasAt`].
    pub canvas: Size,
    /// The pixels of the [`CanvasAt`].
    pub pixels: Box<[Option<C>]>,
}

impl<C: PixelColor> CanvasAt<C> {
    /// Create a new blank [`CanvasAt`].
    pub fn new(top_left: Point, canvas: Size) -> Self {
        let pixels = new_pixels(canvas, None);

        Self {
            top_left,
            canvas,
            pixels,
        }
    }

    /// Create a [`CanvasAt`] filled with a default color.
    pub fn with_default_color(top_left: Point, canvas: Size, default_color: C) -> Self {
        let pixel_count = canvas.width as usize * canvas.height as usize;

        let pixels = vec![Some(default_color); pixel_count].into_boxed_slice();

        Self {
            top_left,
            canvas,
            pixels,
        }
    }

    /// Create a new blank [`CanvasAt`] with a set center on the display.
    pub fn with_center(center: Point, size: Size) -> Self {
        // todo: use our own center_offset like in this assoc. fn
        let rect = Rectangle::with_center(center, size);

        Self::new(rect.top_left, size)
    }

    /// Returns the center of the bounding box.
    pub fn center(&self) -> Point {
        self.bounding_box().center()
    }

    /// Returns the color of the pixel at a given [`Point`].
    ///
    /// Returns [`None`] if the [`Point`] is outside of the [`CanvasAt`].
    pub fn get_pixel(&self, point: Point) -> Option<C> {
        self.point_to_index(point)
            .and_then(|index| self.pixels.get(index).copied().flatten())
    }

    /// Helper method that returns the index in the array of pixels
    fn point_to_index(&self, point: Point) -> Option<usize> {
        point_to_index(self.canvas, self.top_left, point)
    }

    fn index_to_point(&self, index: usize) -> Option<Point> {
        // we account for the displacement of the current Canvas
        index_to_point(self.canvas, index).map(|point| point + self.top_left)
    }

    /// Create a new cropped [`CanvasAt`].
    ///
    /// This method takes into account the top left [`Point`] of the `area`
    /// you'd like to crop relative to the **display**.
    ///
    /// If the width or height of the [`Rectangle`] is `0`, this method will
    /// return [`None`] (see [`Rectangle::bottom_right()`])
    // todo: make safer
    pub fn crop(&self, area: &Rectangle) -> Option<CanvasAt<C>> {
        let mut new = CanvasAt::new(area.top_left, area.size);

        // returns None when width or height is `0`
        // it's safe to return `None` for Canvas too!
        let area_bottom_right = area.bottom_right()?;

        let new_pixels = self.pixels.iter().enumerate().filter_map(|(index, color)| {
            let color = match color {
                Some(color) => *color,
                None => return None,
            };

            let point = self.index_to_point(index).expect("Will never fail");

            // for here on, we should compare the point based on the area we want to crop
            if point >= area.top_left && point <= area_bottom_right {
                let pixel = Pixel(point, color);

                Some(pixel)
            } else {
                None
            }
        });

        new.draw_iter(new_pixels).ok().map(|_| new)
    }
}

impl<C: PixelColor> Dimensions for CanvasAt<C> {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.top_left, self.canvas)
    }
}

impl<C: PixelColor> DrawTarget for CanvasAt<C> {
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

impl<C> Drawable for CanvasAt<C>
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
            // for the drawing position we need to account for the top_left offset of the drawn display
            self.get_pixel(point).map(|color| Pixel(point, color))
        });

        target.draw_iter(pixels_iter)
    }
}

#[cfg(feature = "transform")]
#[cfg_attr(docsrs, doc(cfg(feature = "transform")))]
impl<C: PixelColor> embedded_graphics::transform::Transform for CanvasAt<C> {
    fn translate(&self, by: Point) -> Self {
        Self {
            // update the CanvasAt top_left!
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

/// Returns the center offset.
///
/// The center offset is defined as the offset between the top left corner and
/// the center point of a rectangle with the given size.
fn center_offset(size: Size) -> Size {
    size.saturating_sub(Size::new_equal(1)) / 2
}

/// Generic function that will take into account the top_left offset when returning the index
// TODO: make safer
fn point_to_index(size: Size, top_left_offset: Point, point: Point) -> Option<usize> {
    // we must account for the top-left corner of the drawing box
    if let Ok((x, y)) = <(u32, u32)>::try_from(point - top_left_offset) {
        if x < size.width && y < size.height {
            return Some((x + y * size.width) as usize);
        }
    }

    None
}

fn index_to_point(size: Size, index: usize) -> Option<Point> {
    let x = index as i32 % size.width as i32;
    let y = index as i32 / size.width as i32;
    let point = Point { x, y };

    Some(point)
}

fn new_pixels<C: PixelColor>(size: Size, color: Option<C>) -> Box<[Option<C>]> {
    let pixel_count = size.width as usize * size.height as usize;

    vec![color; pixel_count].into_boxed_slice()
}

#[cfg(test)]
mod test {
    use embedded_graphics_core::pixelcolor::BinaryColor;

    use super::*;

    #[test]
    fn test_index_to_point() {
        let canvas = Canvas::<BinaryColor>::new(Size {
            width: 320,
            height: 240,
        });

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
