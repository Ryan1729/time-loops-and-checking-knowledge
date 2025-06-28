use platform_types::{ARGB, Command, PALETTE, sprite, unscaled, command::{self, Rect}, PaletteIndex, FONT_BASE_Y, FONT_WIDTH};

// TODO make constants for all valid tile IDs
pub type TileId = u16;

#[derive(Default)]
pub struct Commands {
    commands: Vec<Command>,
}

impl Commands {
    pub fn slice(&self) -> &[Command] {
        &self.commands
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn sspr(
        &mut self,
        sprite_xy: sprite::XY,
        rect: command::Rect,
    ) {
        self.commands.push(
            Command {
                sprite_xy,
                rect,
                colour_override: 0,
            }
        );
    }

    pub fn print_char(
        &mut self,
        character: u8,
        x: unscaled::X,
        y: unscaled::Y,
        colour: PaletteIndex
    ) {
        fn get_char_xy(sprite_number: u8) -> sprite::XY {
            type Inner = sprite::Inner;
            let sprite_number = Inner::from(sprite_number);
            const CH_SIZE: Inner = CHAR_SIZE as Inner;
            const SPRITES_PER_ROW: Inner = FONT_WIDTH as Inner / CH_SIZE;

            sprite::XY {
                x: sprite::X(
                    (sprite_number % SPRITES_PER_ROW) * CH_SIZE
                ),
                y: sprite::Y(
                    FONT_BASE_Y as Inner +
                    (sprite_number / SPRITES_PER_ROW) * CH_SIZE
                ),
            }
        }

        let sprite_xy = get_char_xy(character);
        self.commands.push(
            Command {
                sprite_xy,
                rect: Rect::from_unscaled(unscaled::Rect {
                    x,
                    y,
                    w: CHAR_W,
                    h: CHAR_H,
                }),
                colour_override: PALETTE[colour as usize],
            }
        );
    }

    pub fn print(
        &mut self,
        characters: &[u8],
        x: unscaled::X,
        y: unscaled::Y,
        colour: PaletteIndex
    ) {
        let mut current_x = x;

        for &character in characters {
            self.print_char(
                character,
                current_x,
                y,
                colour,
            );

            current_x += CHAR_W;
        }
    }

    pub fn draw_tile(
        &mut self,
        tile_id: TileId,
        x: unscaled::X,
        y: unscaled::Y
    ) {
        self.sspr(
            id_to_xy(tile_id),
            Rect::from_unscaled(unscaled::Rect {
                x,
                y,
                w: tile::WIDTH,
                h: tile::HEIGHT,
            })
        );
    }

    pub fn draw_text_box(
        &mut self,
        min_x: unscaled::X,
        min_y: unscaled::Y,
        max_x: unscaled::X,
        max_y: unscaled::Y,
    ) {
        const TOP_LEFT: TileId = TILES_PER_ROW * 10;
        const TOP: TileId = TOP_LEFT + 1;
        const TOP_RIGHT: TileId = TOP + 1;

        const MIDDLE_LEFT: TileId = TOP_LEFT + TILES_PER_ROW;
        const MIDDLE: TileId = TOP + TILES_PER_ROW;
        const MIDDLE_RIGHT: TileId = TOP_RIGHT + TILES_PER_ROW;

        const BOTTOM_LEFT: TileId = MIDDLE_LEFT + TILES_PER_ROW;
        const BOTTOM: TileId = MIDDLE + TILES_PER_ROW;
        const BOTTOM_RIGHT: TileId = MIDDLE_RIGHT + TILES_PER_ROW;

        let after_left_corner = min_x.saturating_add(tile::WIDTH);
        let before_right_corner = max_x.saturating_sub(tile::WIDTH);

        let below_top_corner = min_y.saturating_add(tile::HEIGHT);
        let above_bottom_corner = max_y.saturating_sub(tile::HEIGHT);

        {
            let mut fill_y = below_top_corner;
            while fill_y < above_bottom_corner {
                let mut fill_x = after_left_corner;
                while fill_x < before_right_corner {
                    self.draw_tile(MIDDLE, fill_x, fill_y);

                    fill_x += tile::WIDTH;
                }
                fill_y += tile::HEIGHT;
            }
        }

        {
            let mut fill_x = after_left_corner;
            while fill_x < before_right_corner {
                self.draw_tile(TOP, fill_x, min_y);
                self.draw_tile(BOTTOM, fill_x, above_bottom_corner);
    
                fill_x += tile::WIDTH;
            }
        }

        {
            let mut fill_y = below_top_corner;
            while fill_y < above_bottom_corner {
                self.draw_tile(MIDDLE_LEFT, min_x, fill_y);
                self.draw_tile(MIDDLE_RIGHT, before_right_corner, fill_y);
    
                fill_y += tile::HEIGHT;
            }
        }

        self.draw_tile(TOP_LEFT, min_x, min_y);
        self.draw_tile(TOP_RIGHT, before_right_corner, min_y);
        self.draw_tile(BOTTOM_LEFT, min_x, above_bottom_corner);
        self.draw_tile(BOTTOM_RIGHT, before_right_corner, above_bottom_corner);
    }
}

const TILES_PER_ROW: TileId = 14;

pub mod tile {
    use super::*;

    use unscaled::{W, H};

    pub const WIDTH: W = W(8);
    pub const HEIGHT: H = H(8);
}

fn id_to_xy(tile_id: TileId) -> sprite::XY {
    type Inner = sprite::Inner;
    let sprite_number = Inner::from(tile_id);

    // + 1 for the apron
    const TILE_SIZE: Inner = tile::WIDTH.get() + 1;

    const TILE_BASE_X: Inner = 1;
    const TILE_BASE_Y: Inner = 1;

    sprite::XY {
        x: sprite::X(
            TILE_BASE_X +
            (sprite_number % TILES_PER_ROW) * TILE_SIZE
        ),
        y: sprite::Y(
            TILE_BASE_Y +
            (sprite_number / TILES_PER_ROW) * TILE_SIZE
        ),
    }
}

pub mod card {
    use super::*;

    use unscaled::{W, H, w_const_add, w_const_sub, h_const_add, h_const_sub};

    pub const WIDTH: W = W(20);
    pub const HEIGHT: H = H(30);

    pub const FRONT_SPRITE_X: u8 = 2;
    pub const FRONT_SPRITE_Y: u8 = 1;

    pub const LEFT_RANK_EDGE_W: W = W(3);
    pub const LEFT_RANK_EDGE_H: H = H(3);

    pub const LEFT_SUIT_EDGE_W: W = W(1);
    pub const LEFT_SUIT_EDGE_H: H = H(10);

    pub const RIGHT_RANK_EDGE_W: W = w_const_sub(
        WIDTH,
        w_const_add(LEFT_RANK_EDGE_W, CHAR_W)
    );
    pub const RIGHT_RANK_EDGE_H: H = h_const_sub(
        HEIGHT,
        h_const_add(LEFT_RANK_EDGE_H, CHAR_H)
    );

    pub const RIGHT_SUIT_EDGE_W: W = w_const_sub(
        WIDTH,
        w_const_add(LEFT_SUIT_EDGE_W, CHAR_W)
    );
    pub const RIGHT_SUIT_EDGE_H: H = h_const_sub(
        HEIGHT,
        h_const_add(LEFT_SUIT_EDGE_H, CHAR_H)
    );
}

pub const CHAR_SIZE: u8 = 8;
pub const CHAR_W: unscaled::W = unscaled::W(CHAR_SIZE as _);
pub const CHAR_H: unscaled::H = unscaled::H(CHAR_SIZE as _);

pub const FONT_FLIP: u8 = 128;

