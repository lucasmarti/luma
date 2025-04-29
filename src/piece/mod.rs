pub struct Piece {
    pub type_: Type,
    pub color: Color,
}
pub enum Type {
    KING,
    QUEEN,
    ROOK,
    PAWN,
    KNIGHT,
    BISHOP,
}
pub enum Color {
    BLACK,
    WHITE,
}
