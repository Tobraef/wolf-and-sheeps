use super::{board::Move, Board, Coord};

trait AbsDiff {
    fn abs_dif(self, other: u8) -> u8;
}

impl AbsDiff for u8 {
    fn abs_dif(self, other: u8) -> u8 {
        if self >= other {
            self - other
        } else {
            other - self
        }
    }
}

pub fn valid_move(board: &Board, mv: &Move) -> bool {
    let from = &mv.from;
    let to = &mv.to;
    if board.wolf == *from {
        from.x.abs_dif(to.x) == 1
            && from.y.abs_dif(to.y) == 1
            && board.sheeps.iter().all(|s| s != to)
    } else {
        from.y.saturating_sub(to.y) == 1 && to.x.abs_dif(from.x) == 1 && board.wolf != *to
    }
}

pub fn all_available_sheeps_moves(board: &Board) -> Vec<Move> {
    board
        .sheeps
        .iter()
        .flat_map(|s| {
            sheep_moves(s)
                .into_iter()
                .map(|x| Move::new(s.clone(), Coord::new(x.0, x.1)))
        })
        .filter(|sheep_move| board.wolf != sheep_move.to)
        .collect()
}

pub fn all_available_wolf_moves(wolf: &Coord, sheeps: &[Coord]) -> Vec<Move> {
    wolf_moves(wolf)
        .iter()
        .map(|w| Coord::new(w.0, w.1))
        .filter(|w| sheeps.iter().all(|s| *s != *w))
        .map(|w| Move::new(wolf.clone(), w))
        .collect()
}

fn sheep_moves(coord: &Coord) -> Vec<(u8, u8)> {
    let first = coord.x;
    let second = coord.y;
    match first {
        0 => match second {
            0 => vec![],
            1..=6 => vec![(first + 1, second - 1)],
            7 => vec![(first + 1, second - 1)],
            _ => panic!("Coord out of bounds"),
        },
        1..=6 => match second {
            0 => vec![],
            1..=6 => vec![(first + 1, second - 1), (first - 1, second - 1)],
            7 => vec![(first - 1, second - 1), (first + 1, second - 1)],
            _ => panic!("Coord out of bounds"),
        },
        7 => match second {
            0 => vec![],
            1..=6 => vec![(first - 1, second - 1)],
            7 => vec![(first - 1, second - 1)],
            _ => panic!("Coord out of bounds"),
        },
        _ => panic!("Coord out of bounds"),
    }
}

fn wolf_moves(coord: &Coord) -> Vec<(u8, u8)> {
    let first = coord.x;
    let second = coord.y;
    match first {
        0 => match second {
            0 => vec![(first + 1, second + 1)],
            1..=6 => vec![(first + 1, second + 1), (first + 1, second - 1)],
            7 => vec![(first + 1, second - 1)],
            _ => panic!("Coord out of bounds"),
        },
        1..=6 => match second {
            0 => vec![(first - 1, second + 1), (first + 1, second + 1)],
            1..=6 => vec![
                (first + 1, second + 1),
                (first + 1, second - 1),
                (first - 1, second + 1),
                (first - 1, second - 1),
            ],
            7 => vec![(first - 1, second - 1), (first + 1, second - 1)],
            _ => panic!("Coord out of bounds"),
        },
        7 => match second {
            0 => vec![(first - 1, second + 1)],
            1..=6 => vec![(first - 1, second + 1), (first - 1, second - 1)],
            7 => vec![(first - 1, second - 1)],
            _ => panic!("Coord out of bounds"),
        },
        _ => panic!("Coord out of bounds"),
    }
}

pub fn wolf_cant_move(board: &Board) -> bool {
    let available_moves = wolf_moves(&board.wolf);
    let mut available_moves = available_moves
        .iter()
        .map(|x| Coord::new(x.0, x.1))
        .filter(|m| board.sheeps.iter().all(|s| s != m));
    available_moves.next().is_none()
}

pub fn move_pin(board: &mut Board, mv: &Move) -> bool {
    if valid_move(board, mv) {
        let to_move = board.iter_mut().find(|p| *p == &mv.from).unwrap();
        *to_move = mv.to.clone();
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Board;

    fn xy(x: u8, y: u8) -> Coord {
        Coord::new(x, y)
    }

    fn mv(x1: u8, y1: u8, x2: u8, y2: u8) -> Move {
        Move::new(xy(x1, y1), xy(x2, y2))
    }

    fn mv_c(a: &Coord, b: &Coord) -> Move {
        Move::new(a.clone(), b.clone())
    }

    #[test]
    fn valid_move_sheep_should_move_only_up() {
        let mut board = Board::default();
        assert!(valid_move(&board, &mv(1, 7, 0, 6)));
        assert!(valid_move(&board, &mv(1, 7, 2, 6)));

        board.sheeps[0] = xy(0, 6);

        assert!(valid_move(&board, &mv(0, 6, 1, 5)));
        assert!(!valid_move(&board, &mv(0, 6, 1, 7)));
        assert!(!valid_move(&board, &mv(0, 6, 2, 6)));
    }

    #[test]
    fn valid_move_wolf_should_move_anywhere() {
        let mut board = Board::default();
        board.wolf = xy(board.wolf.x, board.wolf.y + 2);
        assert!(valid_move(
            &board,
            &mv_c(&board.wolf, &xy(board.wolf.x + 1, board.wolf.y + 1))
        ));
        assert!(valid_move(
            &board,
            &mv_c(&board.wolf, &xy(board.wolf.x + 1, board.wolf.y - 1))
        ));
        assert!(valid_move(
            &board,
            &mv_c(&board.wolf, &xy(board.wolf.x - 1, board.wolf.y + 1))
        ));
        assert!(valid_move(
            &board,
            &mv_c(&board.wolf, &xy(board.wolf.x - 1, board.wolf.y - 1))
        ));

        assert!(!valid_move(
            &board,
            &mv_c(&board.wolf, &xy(board.wolf.x + 3, board.wolf.y))
        ));
        assert!(!valid_move(
            &board,
            &mv_c(&board.wolf, &xy(board.wolf.x + 3, board.wolf.y + 3))
        ));
    }

    #[test]
    fn valid_move_should_disallow_moving_ontop_of_eachother() {
        let mut board = Board::default();
        board.sheeps[0] = xy(2, 1);

        assert!(!valid_move(
            &board,
            &mv_c(&board.sheeps[1], &board.sheeps[0])
        ));

        board.wolf = xy(3, 2);

        assert!(!valid_move(&board, &mv_c(&board.sheeps[0], &board.wolf)));
        assert!(!valid_move(&board, &mv_c(&board.wolf, &board.sheeps[0])));
    }

    #[test]
    fn all_available_sheeps_moves_should_return_all_moves() {
        let board = Board::default();

        let all_moves = all_available_sheeps_moves(&board);

        let all_possible = vec![
            Move::new(board.sheeps[0].clone(), xy(1, 6)),
            Move::new(board.sheeps[1].clone(), xy(1, 6)),
            Move::new(board.sheeps[1].clone(), xy(3, 6)),
            Move::new(board.sheeps[2].clone(), xy(3, 6)),
            Move::new(board.sheeps[2].clone(), xy(5, 6)),
            Move::new(board.sheeps[3].clone(), xy(5, 6)),
            Move::new(board.sheeps[3].clone(), xy(7, 6)),
        ];

        assert_eq!(all_moves.len(), all_possible.len());
        for m in all_moves {
            assert!(
                all_possible.contains(&m),
                "Move {m:?} shouldn't be possible"
            );
        }
    }
}
