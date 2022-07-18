use vec_option::VecOption;
use crate::chess_components::{CPosition, CColor, CKind, Kind, CSprite};

pub type EntityIndex = usize;

pub struct GameState {
    pub c_positions: VecOption<CPosition>,
    pub c_colors: VecOption<CColor>,
    pub c_kinds: VecOption<CKind>,
    pub c_sprites: VecOption<CSprite>,

    pub e_pieces: Vec<EntityIndex>
}

impl GameState {
    fn new() -> GameState {
        GameState {
            c_positions: VecOption::new(),
            c_colors: VecOption::new(),
            c_kinds: VecOption::new(),
            c_sprites: VecOption::new(),
            e_pieces: Vec::new()
        }
    }

    fn add_piece(&mut self, is_white: bool, kind: Kind, x: i32, y: i32) {
        self.e_pieces.push(self.e_pieces.len());
        self.c_colors.push(CColor { is_white });
        self.c_sprites.push(CSprite::from_kind(&kind, &is_white));
        self.c_kinds.push(CKind { kind });
        self.c_positions.push(CPosition { x, y });
    }
}

pub fn initialize_game_state() -> GameState {
    let mut gs = GameState::new();

    // Init pawn
    for i in 0..8 {
        gs.add_piece(false, Kind::Pawn, i, 1);
        gs.add_piece(true, Kind::Pawn, i, 6)
    }

    // Init rooks
    gs.add_piece(false, Kind::Rook, 0, 0);
    gs.add_piece(false, Kind::Rook, 7, 0);
    gs.add_piece(true, Kind::Rook, 0, 7);
    gs.add_piece(true, Kind::Rook, 7, 7);

    // Init knights
    gs.add_piece(false, Kind::Knight, 1, 0);
    gs.add_piece(false, Kind::Knight, 6, 0);
    gs.add_piece(true, Kind::Knight, 1, 7);
    gs.add_piece(true, Kind::Knight, 6, 7);

    // Init bishops
    gs.add_piece(false, Kind::Bishop, 2, 0);
    gs.add_piece(false, Kind::Bishop, 5, 0);
    gs.add_piece(true, Kind::Bishop, 2, 7);
    gs.add_piece(true, Kind::Bishop, 5, 7);

    // Init queens
    gs.add_piece(false, Kind::Queen, 3, 0);
    gs.add_piece(true, Kind::Queen, 3, 7);

    // Init kings
    gs.add_piece(false, Kind::King, 4, 0);
    gs.add_piece(true, Kind::King, 4, 7);

    gs
}
