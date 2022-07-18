use sdl2::rect::Rect;

const SPRITE_SIZE: u32 = 60;

pub struct CPosition { pub x:i32, pub y:i32 }
pub struct CColor { pub is_white:bool }

pub struct CSprite { pub sprite:Rect }
impl CSprite {
    pub fn from_kind(kind: &Kind, is_white: &bool) -> CSprite {
        let y = (if *is_white { 1 } else { 0 }) * SPRITE_SIZE as i32;

        match kind {
            Kind::Pawn => CSprite { sprite: Rect::new((5 * SPRITE_SIZE) as i32, y, SPRITE_SIZE, SPRITE_SIZE) },
            Kind::Bishop => CSprite { sprite: Rect::new((4 * SPRITE_SIZE) as i32, y, SPRITE_SIZE, SPRITE_SIZE) },
            Kind::Knight => CSprite { sprite: Rect::new((3 * SPRITE_SIZE) as i32, y, SPRITE_SIZE, SPRITE_SIZE) },
            Kind::Rook => CSprite { sprite: Rect::new((2 * SPRITE_SIZE) as i32, y, SPRITE_SIZE, SPRITE_SIZE) },
            Kind::Queen => CSprite { sprite: Rect::new((0 * SPRITE_SIZE) as i32, y, SPRITE_SIZE, SPRITE_SIZE) },
            Kind::King => CSprite { sprite: Rect::new((1 * SPRITE_SIZE) as i32, y, SPRITE_SIZE, SPRITE_SIZE) }
        }
    }
}


pub struct CKind { pub kind:Kind }
pub enum Kind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King
}


pub struct CPiece { id:usize }
