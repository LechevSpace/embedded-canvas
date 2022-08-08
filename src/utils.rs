use embedded_graphics_core::geometry::Size;

/// Returns the center offset.
///
/// The center offset is defined as the offset between the top left corner and
/// the center point of a rectangle with the given size.
pub fn center_offset(size: Size) -> Size {
    size.saturating_sub(Size::new_equal(1)) / 2
}
