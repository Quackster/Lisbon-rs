//! Mirrors `com.github.bhlangonijr.chesslib.*` (the external Java chess
//! library; a minimal engine with the same API is implemented here).

/// Mirrors `Side`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side {
    White,
    Black,
}

/// Mirrors `PieceType`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceType {
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// Mirrors `Piece`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    piece_type: PieceType,
    side: Option<Side>,
}

impl Piece {
    /// Mirrors `Piece.make(Side, PieceType)`.
    pub fn make(side: Side, piece_type: PieceType) -> Self {
        Self {
            piece_type,
            side: Some(side),
        }
    }

    /// Mirrors `getPieceType()`.
    pub fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    /// Mirrors `getPieceSide()`.
    pub fn get_piece_side(&self) -> Option<Side> {
        self.side
    }
}

/// Mirrors `Square` (the 64-constant Java enum is modelled as a struct).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Square {
    file: u8,
    rank: u8,
}

impl Square {
    /// Mirrors `Square.values()`.
    pub fn values() -> Vec<Square> {
        (0..64)
            .map(|i| Square {
                file: (i % 8) + 1,
                rank: (i / 8) + 1,
            })
            .collect()
    }

    /// Mirrors `Square.valueOf` (returns `None` on an unknown name; Java
    /// throws).
    pub fn value_of(name: &str) -> Option<Square> {
        let name = name.to_lowercase();
        if name.len() != 2 {
            return None;
        }

        let mut chars = name.chars();
        let file = (chars.next()? as u8).wrapping_sub(b'a');
        let rank = (chars.next()? as u8).wrapping_sub(b'0');

        if file > 7 || rank > 9 {
            return None;
        }

        Some(Square {
            file: file + 1,
            rank: rank + 1,
        })
    }

    /// Mirrors `Square.value()`.
    pub fn value(&self) -> String {
        format!(
            "{}{}",
            (b'a' + self.file - 1) as char,
            self.rank
        )
    }
}

/// Mirrors `Move`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Move {
    from: Square,
    to: Square,
    piece: Option<Piece>,
}

impl Move {
    /// Mirrors the `Move(Square, Square)` constructor.
    pub fn new(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            piece: None,
        }
    }

    /// Mirrors the `Move(Square, Square, Piece)` constructor.
    pub fn new_with_piece(from: Square, to: Square, piece: Piece) -> Self {
        Self {
            from,
            to,
            piece: Some(piece),
        }
    }

    /// Get the source square.
    pub fn get_from(&self) -> Square {
        self.from
    }

    /// Get the target square.
    pub fn get_to(&self) -> Square {
        self.to
    }

    /// Get the promotion piece, if any.
    pub fn get_piece(&self) -> Option<Piece> {
        self.piece.clone()
    }
}

/// Mirrors `MoveGeneratorException`.
#[derive(Debug)]
pub struct MoveGeneratorError;

fn square_index(square: Square) -> usize {
    ((square.rank - 1) * 8 + (square.file - 1)) as usize
}

/// Mirrors `MoveGenerator`.
pub struct MoveGenerator;

const KNIGHT_DELTAS: [[i8; 2]; 8] = [
    [1, 2],
    [2, 1],
    [2, -1],
    [1, -2],
    [-1, -2],
    [-2, -1],
    [-2, 1],
    [-1, 2],
];
const KING_DELTAS: [[i8; 2]; 8] = [
    [0, 1],
    [1, 1],
    [1, 0],
    [1, -1],
    [0, -1],
    [-1, -1],
    [-1, 0],
    [-1, 1],
];
const BISHOP_DIRS: [[i8; 2]; 4] = [
    [1, 1],
    [1, -1],
    [-1, -1],
    [-1, 1],
];
const ROOK_DIRS: [[i8; 2]; 4] = [
    [0, 1],
    [0, -1],
    [1, 0],
    [-1, 0],
];

impl MoveGenerator {
    /// Mirrors `MoveGenerator.generateLegalMoves(Board)`.
    pub fn generate_legal_moves(board: &Board) -> Result<Vec<Move>, MoveGeneratorError> {
        let side = board.get_side_to_move();
        let mut moves: Vec<Move> = Vec::new();

        for square in Square::values() {
            let Some(piece) = board.get_piece(square) else {
                continue;
            };

            if piece.get_piece_side() != Some(side) {
                continue;
            }

            for target in Self::pseudo_targets(board, square, &piece) {
                let move_ = Move::new(square, target);
                let mut board_copy = board.clone();
                board_copy.apply_move(&move_, None);

                if !board_copy.is_in_check(side) {
                    moves.push(move_);
                }
            }
        }

        Ok(moves)
    }

    /// Generate the pseudo-legal (unfiltered) target squares for a piece.
    fn pseudo_targets(board: &Board, from: Square, piece: &Piece) -> Vec<Square> {
        let mut targets: Vec<Square> = Vec::new();

        match piece.get_piece_type() {
            PieceType::Pawn => {
                let direction: i8 = if piece.get_piece_side() == Some(Side::White) {
                    1
                } else {
                    -1
                };

                let forward = Self::offset_square(from, 0, direction);
                if Self::in_board(forward) {
                    if board.get_piece(forward).is_none() {
                        targets.push(forward);

                        let start_rank = if piece.get_piece_side() == Some(Side::White) {
                            2
                        } else {
                            7
                        };
                        if from.rank == start_rank {
                            let two_forward = Self::offset_square(from, 0, 2 * direction);
                            if board.get_piece(two_forward).is_none() {
                                targets.push(two_forward);
                            }
                        }
                    }

                    for file_delta in [1i8, -1] {
                        let capture = Self::offset_square(from, file_delta, direction);
                        if Self::in_board(capture) {
                            if let Some(target_piece) = board.get_piece(capture) {
                                if target_piece.get_piece_side() != piece.get_piece_side() {
                                    targets.push(capture);
                                }
                            }
                        }
                    }
                }
            }
            PieceType::Knight => {
                for delta in KNIGHT_DELTAS {
                    let target = Self::offset_square(from, delta[0], delta[1]);
                    if Self::in_board(target)
                        && board
                            .get_piece(target)
                            .map_or(true, |p| p.get_piece_side() != piece.get_piece_side())
                    {
                        targets.push(target);
                    }
                }
            }
            PieceType::Bishop => {
                Self::sliding_targets(board, from, &BISHOP_DIRS, piece, &mut targets);
            }
            PieceType::Rook => {
                Self::sliding_targets(board, from, &ROOK_DIRS, piece, &mut targets);
            }
            PieceType::Queen => {
                Self::sliding_targets(board, from, &BISHOP_DIRS, piece, &mut targets);
                Self::sliding_targets(board, from, &ROOK_DIRS, piece, &mut targets);
            }
            PieceType::King => {
                for delta in KING_DELTAS {
                    let target = Self::offset_square(from, delta[0], delta[1]);
                    if Self::in_board(target)
                        && board
                            .get_piece(target)
                            .map_or(true, |p| p.get_piece_side() != piece.get_piece_side())
                    {
                        targets.push(target);
                    }
                }
            }
            PieceType::None => {}
        }

        targets
    }

    /// Sliding (bishop/rook/queen) piece targets.
    fn sliding_targets(
        board: &Board,
        from: Square,
        directions: &[[i8; 2]],
        piece: &Piece,
        targets: &mut Vec<Square>,
    ) {
        for delta in directions.iter() {
            let mut file = from.file as i8 + delta[0];
            let mut rank = from.rank as i8 + delta[1];

            while Self::in_board_raw(file, rank) {
                let square = Square {
                    file: file as u8,
                    rank: rank as u8,
                };
                let occupied = board.get_piece(square);

                match occupied {
                    None => targets.push(square),
                    Some(occupant) => {
                        if occupant.get_piece_side() != piece.get_piece_side() {
                            targets.push(square);
                        }
                        break;
                    }
                }

                file += delta[0];
                rank += delta[1];
            }
        }
    }

    fn in_board(square: Square) -> bool {
        (1..=8).contains(&square.file) && (1..=8).contains(&square.rank)
    }

    fn in_board_raw(file: i8, rank: i8) -> bool {
        (1..=8).contains(&file) && (1..=8).contains(&rank)
    }

    fn offset_square(from: Square, file_delta: i8, rank_delta: i8) -> Square {
        Square {
            file: (from.file as i8 + file_delta) as u8,
            rank: (from.rank as i8 + rank_delta) as u8,
        }
    }
}

/// Mirrors `Board`.
#[derive(Clone)]
pub struct Board {
    squares: [Option<Piece>; 64],
    side_to_move: Side,
    halfmove_clock: i32,
}

impl Board {
    /// Mirrors the `Board()` constructor.
    pub fn new() -> Self {
        let mut squares: [Option<Piece>; 64] = std::array::from_fn(|_| None);

        let back_rank: [(PieceType, u8); 8] = [
            (PieceType::Rook, 1),
            (PieceType::Knight, 2),
            (PieceType::Bishop, 3),
            (PieceType::Queen, 4),
            (PieceType::King, 5),
            (PieceType::Bishop, 6),
            (PieceType::Knight, 7),
            (PieceType::Rook, 8),
        ];

        for (piece_type, file) in back_rank.iter() {
            let white = Square {
                file: *file,
                rank: 1,
            };
            squares[square_index(white)] = Some(Piece::make(Side::White, *piece_type));
            let black = Square {
                file: *file,
                rank: 8,
            };
            squares[square_index(black)] = Some(Piece::make(Side::Black, *piece_type));

            let white_pawn = Square {
                file: *file,
                rank: 2,
            };
            squares[square_index(white_pawn)] = Some(Piece::make(Side::White, PieceType::Pawn));
            let black_pawn = Square {
                file: *file,
                rank: 7,
            };
            squares[square_index(black_pawn)] = Some(Piece::make(Side::Black, PieceType::Pawn));
        }

        Self {
            squares,
            side_to_move: Side::White,
            halfmove_clock: 0,
        }
    }

    /// Mirrors `getPiece(Square)`.
    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        self.squares[square_index(square)].clone()
    }

    /// Mirrors `getSideToMove()`.
    pub fn get_side_to_move(&self) -> Side {
        self.side_to_move
    }

    /// Mirrors `doMove(Move, boolean)`.
    pub fn do_move(&mut self, move_: &Move, _announce: bool) {
        let promotion = move_.get_piece();
        self.apply_move(move_, promotion.as_ref());

        let moved_piece = self
            .squares[square_index(move_.get_to())]
            .as_ref()
            .map(|piece| piece.get_piece_type())
            .unwrap_or(PieceType::None);

        if moved_piece == PieceType::Pawn {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }
    }

    /// Apply a move to the board (used by `doMove` and by the legality
    /// filter in `generateLegalMoves`).
    fn apply_move(&mut self, move_: &Move, promotion: Option<&Piece>) {
        let from = move_.get_from();
        let to = move_.get_to();

        let Some(piece) = self.squares[square_index(from)].take() else {
            return;
        };

        let piece = match promotion {
            Some(promotion) => Piece {
                piece_type: promotion.get_piece_type(),
                side: piece.get_piece_side(),
            },
            None => piece,
        };

        self.squares[square_index(to)] = Some(piece);
        self.side_to_move = if self.side_to_move == Side::White {
            Side::Black
        } else {
            Side::White
        };
    }

    /// Whether the given side's king is attacked.
    pub fn is_in_check(&self, side: Side) -> bool {
        let Some(king_square) = Self::king_square(self, side) else {
            return false;
        };

        for square in Square::values() {
            let Some(piece) = self.get_piece(square) else {
                continue;
            };

            if piece.get_piece_side() != Some(opposite(side)) {
                continue;
            }

            if MoveGenerator::pseudo_targets(self, square, &piece).contains(&king_square) {
                return true;
            }
        }

        false
    }

    fn king_square(board: &Board, side: Side) -> Option<Square> {
        for square in Square::values() {
            let Some(piece) = board.get_piece(square) else {
                continue;
            };

            if piece.get_piece_type() == PieceType::King && piece.get_piece_side() == Some(side) {
                return Some(square);
            }
        }

        None
    }

    /// Mirrors `isDraw()`.
    pub fn is_draw(&self) -> bool {
        if self.halfmove_clock >= 100 {
            return true;
        }

        let mut white_material = 0;
        let mut black_material = 0;
        let mut white_minor = 0;
        let mut black_minor = 0;

        for square in Square::values() {
            let Some(piece) = self.get_piece(square) else {
                continue;
            };

            let (material, minor) = match piece.get_piece_type() {
                PieceType::Knight | PieceType::Bishop => (1, 1),
                PieceType::Rook => (5, 0),
                PieceType::Queen => (9, 0),
                _ => (0, 0),
            };

            match piece.get_piece_side() {
                Some(Side::White) => {
                    white_material += material;
                    white_minor += minor;
                }
                Some(Side::Black) => {
                    black_material += material;
                    black_minor += minor;
                }
                None => {}
            }
        }

        white_material == 0
            && black_material == 0
            || (white_material <= 1
                && black_material == 0
                && (white_minor <= 1 || white_material == 0))
            || (black_material <= 1
                && white_material == 0
                && (black_minor <= 1 || black_material == 0))
    }

    /// Mirrors `isStaleMate()`.
    pub fn is_stale_mate(&self) -> bool {
        !self.is_in_check(self.side_to_move)
            && MoveGenerator::generate_legal_moves(self).map_or(true, |moves| moves.is_empty())
    }

    /// Mirrors `isMated()`.
    pub fn is_mated(&self) -> bool {
        self.is_in_check(self.side_to_move)
            && MoveGenerator::generate_legal_moves(self).map_or(true, |moves| moves.is_empty())
    }

    /// Mirrors `isPromoRank(Side, Move)`.
    pub fn is_promo_rank(&self, side: Side, move_: &Move) -> bool {
        let rank = if side == Side::White {
            8
        } else {
            1
        };

        move_.get_to().rank == rank
    }
}

fn opposite(side: Side) -> Side {
    if side == Side::White {
        Side::Black
    } else {
        Side::White
    }
}
