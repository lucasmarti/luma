#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub column: Column,
    pub row: Row,
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub enum Column {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl Column {
    pub fn from(index: u32) -> Option<Column> {
        match index {
            1 => Some(Column::A),
            2 => Some(Column::B),
            3 => Some(Column::C),
            4 => Some(Column::D),
            5 => Some(Column::E),
            6 => Some(Column::F),
            7 => Some(Column::G),
            8 => Some(Column::H),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub enum Row {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
}

impl Row {
    pub fn from(index: u32) -> Option<Row> {
        match index {
            1 => Some(Row::_1),
            2 => Some(Row::_2),
            3 => Some(Row::_3),
            4 => Some(Row::_4),
            5 => Some(Row::_5),
            6 => Some(Row::_6),
            7 => Some(Row::_7),
            8 => Some(Row::_8),
            _ => None,
        }
    }
}
