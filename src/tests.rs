
#[cfg(test)]
mod tests {
    use crate::{Board, CheckersColor, MoveExecutor, Piece};
    use crate::checkers_utils::CheckersError;
    use crate::moves::{Jump, SimpleMove};

    #[test]
    fn set_element_test() {
        println!("=== Set element test ===");
        let mut board = Board::empty();
        let res1 = board.set_at(0, 1, Board::BLACK_QUEEN);
        let res2 = board.set_at(1, 0, Board::BLACK_PAWN);
        println!("Board with a BLACK pawn:\n{}", board.repr());
        let res2 = board.set_at(1, 0, Board::WHITE_PAWN);
        println!("Board with a WHITE pawn:\n{}", board.repr());
        assert_eq!(board.get_board(), 0b0001_0000_0000_0000_0111);
        assert_eq!(res1, Ok(()));
        assert_eq!(res2, Ok(()));

        let res3 = board.set_at(0, 0, Board::WHITE_QUEEN);
        let res4 = board.set_at(10, 1, Board::BLACK_PAWN);
        let res5 = board.set_at(0, 1, 0b1111);
        assert_eq!(res3, Err(CheckersError::RuleError));
        assert_eq!(res4, Err(CheckersError::IndexOutOfBounds));
        assert_eq!(res5, Err(CheckersError::PawnBinaryValueError));
    }

    #[test]
    fn board_new_constructor_test() {
        let res = match Board::new(2) {
            Ok(_) => true,
            Err(_) => false,
        };
        assert!(res);

        let res = match Board::new(4) {
            Ok(_) => true,
            Err(_) => false,
        };
        assert!(!res);
    }

    #[test]
    fn get_element_test() {
        let board = Board::new(2).unwrap();
        let elem = board.get_at(0, 1);
        let res = match elem {
            Ok(Some(Piece::Pawn(CheckersColor::Black))) => true,
            _ => false,
        };
        assert!(res);

        let elem = board.get_at(0, 10);
        let res = match elem {
            Err(CheckersError::IndexOutOfBounds) => true,
            _ => false,
        };
        assert!(res);
    }

    #[test]
    fn capture_test() {
        println!("=== Capture test ===");
        let mut board = Board::empty();
        let _ = board.set_at(4, 1, Board::BLACK_PAWN);
        let _ = board.set_at(3, 2, Board::WHITE_QUEEN);
        let _ = board.set_at(1, 2, Board::WHITE_PAWN);
        let captures = vec![
            Jump::new(4, 1, 2, 3, 3, 2),
            Jump::new(2, 3, 0, 1, 1, 2)
        ];
        println!("Before capture:\n{}", board.repr());
        board = MoveExecutor::execute_capture(&mut board, &captures);
        println!("After capture:\n{}", board.repr());
        assert_eq!(board.get_board(), 0b0011_u128);
    }

    #[test]
    fn move_test() {
        println!("=== Move test ===");
        let mut board = Board::empty();
        let _ = board.set_at(1, 2, Board::WHITE_PAWN);
        println!("Before move:\n{}", board.repr());
        board = MoveExecutor::execute_move(board, SimpleMove::new(1, 2, 0, 3));
        println!("After move:\n{}", board.repr());
        assert_eq!(board.get_board(), 0b0001_0000);
    }

    #[test]
    fn set_field_excluded_test() {
        let mut board = Board::empty();
        let _ = board.set_field_excluded(0, 3);
        let _ = board.set_at(0, 1, Board::WHITE_QUEEN);
        let _ = board.set_field_excluded(0, 1);
        assert_eq!(board.get_board(), 0b1000_1101);

        let res = board.set_field_excluded(10, 0);
        assert_eq!(res, Err(CheckersError::IndexOutOfBounds));

        let mut board = Board::empty();
        let _ = board.set_field_excluded(7, 6);
        assert_eq!(board.get_board(), 0b1000_u128 << 31 * 4);
    }

    #[test]
    fn is_field_excluded_test() {
        let mut board = Board::new(2).unwrap();
        let _ = board.set_field_excluded(0, 1);
        assert!(board.is_field_excluded(0, 1).unwrap());

        let _ = board.set_field_excluded(5, 6);
        let res = board.is_field_excluded(5, 6);
        assert_eq!(res, Ok(true));

        let _ = board.set_field_excluded(0, 1);
        let res = board.is_field_excluded(10, 0);
        assert_eq!(res, Err(CheckersError::IndexOutOfBounds));
    }

    #[test]
    fn reset_excluded_fields_test() {
        let mut board = Board::empty();
        let _ = board.set_field_excluded(1, 2);
        let _ = board.set_field_excluded(7, 4);
        board.reset_excluded_fields();
        assert_eq!(board.get_board(), 0);
    }

    #[test]
    fn is_empty_at_test() {
        let board = Board::new(2).unwrap();
        assert!(!board.is_empty_at(0, 1).unwrap());
        assert!(board.is_empty_at(2, 3).unwrap());
        assert_eq!(board.is_empty_at(10, 0), Err(CheckersError::IndexOutOfBounds));
        assert_eq!(board.is_empty_at(2, 4), Err(CheckersError::RuleError));
    }
}