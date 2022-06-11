//! `Canvas`es implemented with const generics
use embedded_graphics_core::{
    prelude::{
        Dimensions, DrawTarget, Drawable, OriginDimensions, Pixel, PixelColor, Point, PointsIter,
        Size,
    },
    primitives::Rectangle,
};

use crate::canvas::center_offset;

/// Canvas on which you can draw but it's not drawable on the display yet.
/// Implemented using [const generics][const_generics_rfc].
///
/// Draw on the [`CCanvas`] using origin of [`Point::zero()`].
///
/// The width (`W`) and height (`H`) constants of the [`CCanvas`]
/// should less than [`u32::MAX`] as [`Size`] uses [`u32`].
/// 
/// [const_generics_rfc]: https://rust-lang.github.io/rfcs/2000-const-generics.html
pub struct CCanvas<C: PixelColor, const W: usize, const H: usize> {
    // we also store the size for working with embedded-graphics
    pub size: Size,
    pub pixels: [[Option<C>; W]; H],
}

impl<C, const W: usize, const H: usize> CCanvas<C, W, H>
where
    C: PixelColor,
{
    /// Create a new blank [`CCanvas`].
    ///
    /// # Panics
    ///
    /// If either the width (`W`) or heigh (`H`) is larger than [`u32::MAX`]
    /// due to the internal [`Size`] used for implementing [`OriginDimensions`].
    pub fn new() -> Self {
        Self {
            size: Size::new(W as u32, H as u32),
            pixels: [[None; W]; H],
        }
    }

    /// Create a [`CCanvas`] filled with a default color.
    ///
    /// # Panics
    ///
    /// If either the width (`W`) or heigh (`H`) is larger than [`u32::MAX`]
    /// due to the internal [`Size`] used for implementing [`OriginDimensions`].
    pub fn with_default_color(default_color: C) -> Self {
        Self {
            size: Size::new(W as u32, H as u32),
            pixels: [[Some(default_color); W]; H],
        }
    }

    /// Returns the color of the pixel at a given [`Point`].
    ///
    /// Returns [`None`] if the [`Point`] is outside of the [`CCanvas`].
    pub fn get_pixel(&self, point: Point) -> Option<C> {
        // `Point` implements TryFrom only for `u32` & `i32`.
        let x = usize::try_from(point.x).ok()?;
        let y = usize::try_from(point.y).ok()?;

        self.pixels
            .get(x)
            .and_then(|x_row| x_row.get(y))
            .copied()
            .flatten()
    }

    /// Returns the center of [`Size`] of the [`CCanvas`].
    pub fn center(&self) -> Point {
        Point::zero() + center_offset(self.size)
    }

    /// Create a new cropped [`CCanvas`].
    ///
    /// This method takes into account the top left [`Point`] of the `area`
    /// you'd like to crop relative to the [`CCanvas`] itself.
    //
    /// If the width or height of the [`Rectangle`] is `0`, this method will
    /// return [`None`] (see [`Rectangle::bottom_right()`])
    // todo: make safer
    pub fn crop<const NW: usize, const NH: usize>(
        &self,
        area: &Rectangle,
    ) -> Option<CCanvas<C, NW, NH>> {
        let mut new = CCanvas::<C, NW, NH>::new();

        // returns None when width or height is `0`
        // it's safe to return `None` for Canvas too!
        let area_bottom_right = area.bottom_right()?;

        let new_pixels = self.pixels.iter().enumerate().map(|(x, x_row)| {
            
            x_row.iter().enumerate().filter_map(move |(y, color)| {
                let color = match color {
                    Some(color) => *color,
                    None => return None,
                };

                // Canvas always starts from `Point::zero()`
                // todo: make safer
                let point = Point::new(x as i32, y as i32);

                // for here on, we should compare the point based on the area we want to crop
                if point >= area.top_left && point <= area_bottom_right {
                    // remove the area top_left to make the origin at `Point::zero()` for the cropped part
                    let pixel = Pixel(point - area.top_left, color);

                    Some(pixel)
                } else {
                    None
                }
            })
        }).flatten();

        new.draw_iter(new_pixels).ok().map(|_| new)
    }

    /// Sets the place with top left offset where the canvas will be drawn to the display.
    pub fn place_at(&self, top_left: Point) -> CCanvasAt<C, W, H> {
        CCanvasAt {
            top_left,
            size: self.size,
            pixels: self.pixels,
        }
    }

    /// Sets the center of the [`CCanvas`] where it will be drawn to the display.
    pub fn place_center(&self, center: Point) -> CCanvasAt<C, W, H> {
        let top_left = center - center_offset(self.size);

        self.place_at(top_left)
    }
}

impl<C: PixelColor, const W: usize, const H: usize> OriginDimensions for CCanvas<C, W, H> {
    fn size(&self) -> Size {
        self.size
    }
}

impl<C: PixelColor, const W: usize, const H: usize> DrawTarget for CCanvas<C, W, H> {
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            match (usize::try_from(point.x).ok(), usize::try_from(point.y).ok()) {
                (Some(x), Some(y)) => {
                    self.pixels[x][y] = Some(color);
                }
                // if Pixel is outside of the canvas, skip it
                _ => {}
            };
        }

        Ok(())
    }
}

/// Canvas which is drawable at the provided [`Point`] (location) on the display.
#[derive(Debug, Clone, Copy)]
pub struct CCanvasAt<C: PixelColor, const W: usize, const H: usize> {
    /// The top left offset where the [`CCanvasAt`] will be drawn to the display.
    pub top_left: Point,
    /// The size of the [`CCanvasAt`].
    size: Size,
    /// The pixels of the [`CCanvasAt`].
    pub pixels: [[Option<C>; W]; H],
}

impl<C, const W: usize, const H: usize> CCanvasAt<C, W, H>
where
    C: PixelColor,
{
    /// Create a new blank [`CCanvasAt`].
    ///
    /// # Panics
    ///
    /// If either of width (`W`) or heigh (`H`) is larger than [`u32::MAX`]
    /// due to the internal [`Size`] used for implementing [`Dimensions`].
    pub fn new(top_left: Point) -> Self {
        Self {
            top_left,
            size: Size::new(W as u32, H as u32),
            pixels: [[None; W]; H],
        }
    }

    /// Create a [`CCanvasAt`] filled with a default color.
    ///
    /// If either of width (`W`) or heigh (`H`) is larger than [`u32::MAX`]
    /// due to the internal [`Size`] used for implementing [`Dimensions`].
    pub fn with_default_color(top_left: Point, default_color: C) -> Self {
        Self {
            top_left,
            size: Size::new(W as u32, H as u32),
            pixels: [[Some(default_color); W]; H],
        }
    }

    /// Create a new blank [`CCanvasAt`] with a set center on the display.
    ///
    /// # Panics
    ///
    /// If either of width (`W`) or heigh (`H`) is larger than [`u32::MAX`]
    /// due to the internal [`Size`] used for implementing [`Dimensions`].
    pub fn with_center(center: Point) -> Self {
        let top_left = center - center_offset(Size::new(W as u32, H as u32));

        Self::new(top_left)
    }

    /// Returns the center of the bounding box.
    pub fn center(&self) -> Point {
        self.bounding_box().center()
    }

    /// Returns the color of the pixel at a given [`Point`].
    ///
    /// Returns [`None`] if the [`Point`] is outside of the [`CCanvasAt`].
    pub fn get_pixel(&self, point: Point) -> Option<C> {
        // account for the top_left offset of the CanvasAt
        let point_adjusted = point - self.top_left;

        // `Point` implements TryFrom only for `u32` & `i32`.
        let x = usize::try_from(point_adjusted.x).ok()?;
        let y = usize::try_from(point_adjusted.y).ok()?;

        self.pixels
            .get(x)
            .and_then(|x_row| x_row.get(y))
            .copied()
            .flatten()
    }

    // /// Create a new cropped [`CCanvasAt`].
    // ///
    // /// This method takes into account the top left [`Point`] of the `area`
    // /// you'd like to crop relative to the **display**.
    // ///
    // /// If the width or height of the [`Rectangle`] is `0`, this method will
    // /// return [`None`] (see [`Rectangle::bottom_right()`])
    // // todo: make safer
    // pub fn crop(&self, area: &Rectangle) -> Option<CanvasAt<C>> {
    //     let mut new = CanvasAt::new(area.top_left, area.size);

    //     // returns None when width or height is `0`
    //     // it's safe to return `None` for Canvas too!
    //     let area_bottom_right = area.bottom_right()?;

    //     let new_pixels = self.pixels.iter().enumerate().filter_map(|(index, color)| {
    //         let color = match color {
    //             Some(color) => *color,
    //             None => return None,
    //         };

    //         let point = self.index_to_point(index).expect("Will never fail");

    //         // for here on, we should compare the point based on the area we want to crop
    //         if point >= area.top_left && point <= area_bottom_right {
    //             let pixel = Pixel(point, color);

    //             Some(pixel)
    //         } else {
    //             None
    //         }
    //     });

    //     new.draw_iter(new_pixels).ok().map(|_| new)
    // }
}

impl<C: PixelColor, const W: usize, const H: usize> Dimensions for CCanvasAt<C, W, H> {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.top_left, self.size)
    }
}

impl<C: PixelColor, const W: usize, const H: usize> DrawTarget for CCanvasAt<C, W, H> {
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            // account for the top_left offset of the CanvasAt
            let point_adjusted = point - self.top_left;

            match (
                usize::try_from(point_adjusted.x).ok(),
                usize::try_from(point_adjusted.y).ok(),
            ) {
                (Some(x), Some(y)) => {
                    self.pixels[x][y] = Some(color);
                }
                // skip if:
                // - Pixel's Point is outside of the canvas
                // - Pixel's Point is not `usize` representable
                _ => {}
            };
        }

        Ok(())
    }
}

impl<C, const W: usize, const H: usize> Drawable for CCanvasAt<C, W, H>
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
impl<C: PixelColor, const W: usize, const H: usize> embedded_graphics::transform::Transform
    for CCanvasAt<C, W, H>
{
    fn translate(&self, by: Point) -> Self {
        Self {
            // update the CanvasAt top_left!
            top_left: self.top_left + by,
            size: self.size,
            pixels: self.pixels,
        }
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.top_left += by;

        self
    }
}
