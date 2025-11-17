use flo_canvas::TextureId;
#[derive(Clone)]
pub struct Icon {
    pub texture_id: TextureId,
    pub bytes: &'static [u8],
}
impl Icon {
    pub const BLACK_KING: Icon = Icon {
        texture_id: TextureId(0),
        bytes: include_bytes!["icons/black-king.png"],
    };
    pub const BLACK_QUEEN: Icon = Icon {
        texture_id: TextureId(1),
        bytes: include_bytes!["icons/black-queen.png"],
    };
    pub const BLACK_ROOK: Icon = Icon {
        texture_id: TextureId(2),
        bytes: include_bytes!["icons/black-rook.png"],
    };
    pub const BLACK_PAWN: Icon = Icon {
        texture_id: TextureId(3),
        bytes: include_bytes!["icons/black-pawn.png"],
    };
    pub const BLACK_KNIGHT: Icon = Icon {
        texture_id: TextureId(4),
        bytes: include_bytes!["icons/black-knight.png"],
    };
    pub const BLACK_BISHOP: Icon = Icon {
        texture_id: TextureId(5),
        bytes: include_bytes!["icons/black-bishop.png"],
    };

    pub const WHITE_KING: Icon = Icon {
        texture_id: TextureId(6),
        bytes: include_bytes!["icons/white-king.png"],
    };
    pub const WHITE_QUEEN: Icon = Icon {
        texture_id: TextureId(7),
        bytes: include_bytes!["icons/white-queen.png"],
    };
    pub const WHITE_ROOK: Icon = Icon {
        texture_id: TextureId(8),
        bytes: include_bytes!["icons/white-rook.png"],
    };
    pub const WHITE_PAWN: Icon = Icon {
        texture_id: TextureId(9),
        bytes: include_bytes!["icons/white-pawn.png"],
    };
    pub const WHITE_KNIGHT: Icon = Icon {
        texture_id: TextureId(10),
        bytes: include_bytes!["icons/white-knight.png"],
    };
    pub const WHITE_BISHOP: Icon = Icon {
        texture_id: TextureId(11),
        bytes: include_bytes!["icons/white-bishop.png"],
    };
    pub const NEW_GAME_WHITE: Icon = Icon {
        texture_id: TextureId(12),
        bytes: include_bytes!["icons/new-white.png"],
    };
    pub const NEW_GAME_BLACK: Icon = Icon {
        texture_id: TextureId(13),
        bytes: include_bytes!["icons/new-black.png"],
    };
}
