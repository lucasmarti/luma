#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Container {
    pub x_horizontal_min: f32,
    pub x_horizontal_max: f32,
    pub y_vertical_min: f32,
    pub y_vertical_max: f32,
}

impl Container {
    pub fn new(x_horizontal_min: f32, y_vertical_min: f32, height: f32, width: f32) -> Self {
        Container {
            x_horizontal_min,
            x_horizontal_max: x_horizontal_min + width,
            y_vertical_min,
            y_vertical_max: y_vertical_min + height,
        }
    }

    pub fn get_center_x(&self) -> f32 {
        (self.x_horizontal_min + self.x_horizontal_max) / 2.0
    }
    pub fn get_center_y(&self) -> f32 {
        (self.y_vertical_min + self.y_vertical_max) / 2.0
    }
    pub fn get_height(&self) -> f32 {
        self.y_vertical_max - self.y_vertical_min
    }
}
