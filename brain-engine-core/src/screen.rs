use bevy::prelude::*;

/// Describes the screen dimensions and tile sizing, providing helpers for
/// converting tile coordinates into pixel positions.
#[derive(Debug, Clone, Resource)]
pub struct Screen {
    dimensions: UVec2,
    tile_size: f32,
    center_offset: Vec2,
}

impl Screen {
    /// Creates a new [`Screen`].
    ///
    /// * `dimensions` - The number of tiles that fit horizontally and vertically.
    /// * `tile_size` - The size in pixels of a single tile.
    pub fn new(dimensions: UVec2, tile_size: f32) -> Self {
        let center_offset = Vec2::new(
            (dimensions.x as f32 - 1.0) / 2.0 * tile_size,
            (dimensions.y as f32 - 1.0) / 2.0 * tile_size,
        );

        Self {
            dimensions,
            tile_size,
            center_offset,
        }
    }

    /// Converts a tile coordinate into the centered pixel position on screen.
    pub fn pixel_position(&self, tile_position: IVec2) -> Vec3 {
        Vec3::new(
            tile_position.x as f32 * self.tile_size - self.center_offset.x,
            tile_position.y as f32 * self.tile_size - self.center_offset.y,
            0.0,
        )
    }

    /// Returns the number of tiles across the screen.
    pub fn dimensions(&self) -> UVec2 {
        self.dimensions
    }

    /// Returns the size of a tile in pixels.
    pub fn tile_size(&self) -> f32 {
        self.tile_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_position_centers_square_grid() {
        let screen = Screen::new(UVec2::new(5, 5), 64.0);

        assert_eq!(screen.pixel_position(IVec2::new(0, 0)), Vec3::new(-128.0, -128.0, 0.0));
        assert_eq!(screen.pixel_position(IVec2::new(2, 2)), Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(screen.pixel_position(IVec2::new(4, 4)), Vec3::new(128.0, 128.0, 0.0));
    }

    #[test]
    fn pixel_position_handles_rectangular_grid() {
        let screen = Screen::new(UVec2::new(4, 6), 32.0);

        assert_eq!(screen.pixel_position(IVec2::new(0, 0)), Vec3::new(-48.0, -80.0, 0.0));
        assert_eq!(screen.pixel_position(IVec2::new(3, 5)), Vec3::new(48.0, 80.0, 0.0));
    }
}
