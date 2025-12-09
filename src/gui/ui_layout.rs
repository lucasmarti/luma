#[derive(Debug, Clone, Copy)]
pub struct MenuLayout {
    layout: Layout,
}

impl MenuLayout {
    pub fn get(&self, column: Column) -> Container {
        self.layout.cell(1, column.as_index()).unwrap()
    }
    pub fn new(container: Container) -> Self {
        MenuLayout {
            layout: Layout::new(container, 1, 16),
        }
    }
}
pub enum Column {
    Id_1,
    Id_2,
    Id_3,
    Id_4,
    Id_5,
    Id_6,
    Id_7,
    Id_8,
    Id_9,
    Id_10,
    Id_11,
    Id_12,
    Id_13,
    Id_14,
    Id_15,
    Id_16,
}

impl Column {
    fn as_index(&self) -> u8 {
        match self {
            Column::Id_1 => 1,
            Column::Id_2 => 2,
            Column::Id_3 => 3,
            Column::Id_4 => 4,
            Column::Id_5 => 5,
            Column::Id_6 => 6,
            Column::Id_7 => 7,
            Column::Id_8 => 8,
            Column::Id_9 => 9,
            Column::Id_10 => 10,
            Column::Id_11 => 11,
            Column::Id_12 => 12,
            Column::Id_13 => 13,
            Column::Id_14 => 14,
            Column::Id_15 => 15,
            Column::Id_16 => 16,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Container {
    pub x_horizontal_min: f32,
    pub x_horizontal_max: f32,
    pub y_vertical_min: f32,
    pub y_vertical_max: f32,
}
#[derive(Debug, Clone, Copy)]
struct Layout {
    container: Container,
    cell_width: f32,
    cell_height: f32,
    rows: u8,
    columns: u8,
}
impl Layout {
    fn new(container: Container, rows: u8, columns: u8) -> Self {
        let cell_height = (container.y_vertical_max - container.y_vertical_min) / rows as f32;
        let cell_width = (container.x_horizontal_max - container.x_horizontal_min) / columns as f32;

        Layout {
            container,
            cell_width,
            cell_height,
            rows,
            columns,
        }
    }

    fn cell(&self, row: u8, column: u8) -> Option<Container> {
        if (row < 1) || (row > self.rows) || (column < 1) || (column > self.columns) {
            return None;
        }
        Some(Container {
            x_horizontal_min: self.container.x_horizontal_min
                + ((column - 1) as f32 * self.cell_width),
            x_horizontal_max: self.container.x_horizontal_min + (column as f32 * self.cell_width),
            y_vertical_min: self.container.y_vertical_min + ((row - 1) as f32 * self.cell_height),
            y_vertical_max: self.container.y_vertical_min + (row as f32 * self.cell_height),
        })
    }
}

#[cfg(test)]
#[test]
fn test_layouts() {
    let container = Container {
        x_horizontal_min: 400.0,
        x_horizontal_max: 700.0,
        y_vertical_min: 50.0,
        y_vertical_max: 100.0,
    };
    let layout: Layout = Layout::new(container, 5, 3);
    assert_eq!(
        layout.cell(1, 1),
        Some(Container {
            x_horizontal_min: 400.0,
            x_horizontal_max: 500.0,
            y_vertical_min: 50.0,
            y_vertical_max: 60.0,
        })
    );
    assert_eq!(layout.cell(40, 2), None);
}
