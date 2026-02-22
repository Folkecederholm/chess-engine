use crate::types::defs::*;

impl Board {
    pub fn empty() -> Self {
        let board: [[Tile; 8]; 8] = std::array::from_fn(|_| std::array::from_fn(|_| Tile::empty()));
        Self {
            grid: board,
            turn_to_play: Colour::White,
            variant: ChessVariant::Chess,
            passant_square: None,
            fifty_move_rule: 0,
            moves: 0,
            castling_rights: CastlingRights {
                castling_rights: [None, None, None, None],
            },
        }
    }
}

impl Board {
    pub fn set_passant(&mut self, tile: Option<Coord>) {
        self.passant_square = tile;
    }
    pub fn set_fifty_moves(&mut self, moves: u32) {
        self.fifty_move_rule = moves;
    }
    pub fn set_whole_moves(&mut self, moves: u32) {
        self.moves = moves;
    }
    pub fn increment_whole_moves(&mut self) {
        self.moves += 1;
    }
    pub fn get_tile(&self, coord: Coord) -> Tile {
        self.grid[coord.zero_indexed().1][coord.zero_indexed().0]
    }
    pub fn remove_piece(&mut self, coord: Coord) {
        self.grid[coord.zero_indexed().1][coord.zero_indexed().0] = Tile { piece: None };
    }
    pub fn get_castling_rights(&self) -> [Option<Coord>; 4] {
        self.castling_rights.castling_rights
    }
    pub fn get_colour_turn(&self) -> Colour {
        self.turn_to_play
    }
    pub fn remove_castling_rights_colour(&mut self, colour: Colour) {
        let rank = match colour {
            Colour::White => 1,
            Colour::Black => 8,
        };
        for x in 1..8 {
            self.remove_castling_rights_coord(Coord::xy(x, rank));
            // println!("coord: {}", Coord::xy(x, rank));
        }
    }
    pub fn remove_castling_rights_coord(&mut self, coord: Coord) {
        // Feels tautological
        let castling_rights = self.castling_rights.castling_rights;
        // This will automatically clone
        let mut new_castling_rights = castling_rights;
        for (i, right) in castling_rights.iter().enumerate() {
            let Some(castling_right) = right else {
                continue;
            };
            // If it's the coord we want to remove
            if *castling_right == coord {
                // new_castling_rights[i] = Some(castling_right);
                new_castling_rights[i] = None;
            }
        }
        self.set_castling(new_castling_rights);
    }
    pub fn get_passant_square(&self) -> Option<Coord> {
        self.passant_square
    }
    pub fn get_king(&self) -> Coord {
        let king = Piece {
            piece_type: PieceType::King,
            colour: match self.get_colour_turn() {
                Colour::White => Colour::Black,
                Colour::Black => Colour::White,
            },
            // colour: self.get_colour_turn(),
        };
        for col in 1..=8 {
            for row in 1..=8 {
                let Tile { piece: Some(piece) } = self.get_tile(Coord::xy(col, row)) else {
                    continue;
                };
                if piece == king {
                    return Coord::xy(col, row);
                }
            }
        }
        unreachable!()
    }
    pub fn set_piece(&mut self, coord: Coord, piece: Piece) -> Result<(), &'static str> {
        // self.grid[coord.x][coord.y] = Tile { piece: Some(piece) };
        if coord.x > 8 || coord.y > 8 {
            return Err("Coord out of bounds!");
        }
        self.grid[coord.zero_indexed().1][coord.zero_indexed().0] = Tile { piece: Some(piece) };
        Ok(())
    }
    // pub fn print(&self) {}
    pub fn drain(&mut self) {
        for row in &mut self.grid {
            for tile in row.iter_mut() {
                *tile = Tile::empty();
            }
        }
    }
    pub fn set_to_move(&mut self, colour: Colour) {
        self.turn_to_play = colour;
    }
    pub fn set_castling(&mut self, tiles: [Option<Coord>; 4]) {
        let mut castling_rights: [Option<Coord>; 4] = [None, None, None, None];
        for (i, tile) in tiles.iter().enumerate() {
            match tile {
                None => {
                    break;
                }
                Some(n) => {
                    castling_rights[i] = Some(*n);
                }
            }
        }
        self.castling_rights = CastlingRights { castling_rights };
    }
    pub fn tile_is_empty(&self, x: usize, y: usize) -> bool {
        self.get_tile(Coord::xy(x, y)).get_piece().is_none()
    }
    fn tile_has_colour(&self, x: usize, y: usize, colour: Colour) -> bool {
        let Some(tile) = self.get_tile(Coord::xy(x, y)).get_piece() else {
            return false;
        };
        tile.colour == colour
    }
    pub fn tile_can_be_taken(&self, x: usize, y: usize) -> bool {
        self.tile_has_colour(x, y, {
            // If the tile has the colour whose turn it isn't
            let mut c = self.get_colour_turn();
            c.switch();
            c
        })
    }
}
