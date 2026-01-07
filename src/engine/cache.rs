use std::collections::HashMap;

use crate::engine::{evaluation::Evaluation, position::Position};

pub type Cache = HashMap<Position, Evaluation>;
