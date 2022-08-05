
#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use crate::{Board, CheckersColor, MoveExecutor, Piece};
    use crate::checkers_utils::CheckersError;
    use crate::moves::{Jump, Move, SimpleMove};

    #[test]
    fn set_element_test() {
        println!("=== Set element test ===");
        let mut board = Board::empty();
        let res1 = board.set_at(0, 1, Board::BLACK_QUEEN);
        let _ = board.set_at(1, 0, Board::BLACK_PAWN);
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
            Jump::new(4, 1, 2, 3, 3, 2).unwrap(),
            Jump::new(2, 3, 0, 1, 1, 2).unwrap()
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
        board = MoveExecutor::execute_move(board, SimpleMove::new(1, 2, 0, 3).unwrap());
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

    #[test]
    fn get_pieces_test() {
        let board = Board::from_mockup([
            ["", "BP", "", "BP", "", "BP", "", ""],
            ["WP", "", "WP", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "BQ", "", "", "", ""],
            ["", "", "", "", "", "", "", ""]]);
        let pieces = MoveExecutor::get_pieces(&board, CheckersColor::Black);
        let res: Vec<(usize, usize)> = vec![(0, 1), (0, 3), (0, 5), (6, 3)];
        for piece in pieces {
            assert!(res.contains(&piece));
        }

        let pieces = MoveExecutor::get_pieces(&board, CheckersColor::White);
        let res: Vec<(usize, usize)> = vec![(1, 0), (1, 2)];
        for piece in pieces {
            assert!(res.contains(&piece));
        }
    }

    #[test]
    fn get_capturing_pieces_test() {
        let board = Board::from_mockup([
            ["  ", "BP", "  ", "BP", "  ", "BP", "  ", "  "],
            ["WP", "  ", "WP", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "WQ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "BQ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "]]);
        let pieces = MoveExecutor::get_pieces(&board, CheckersColor::Black);
        let (cap_p, cap_q) = MoveExecutor::get_capturing_pieces(&board, &pieces, CheckersColor::Black);
        let p_res: Vec<(usize, usize)> = vec![(0, 1), (0, 3)];
        let q_res: Vec<(usize, usize)> = vec![(6, 3)];
        for pawn in cap_p {
            assert!(p_res.contains(&pawn));
        }
        for queen in cap_q {
            assert!(q_res.contains(&queen));
        }
    }

    #[test]
    fn queen_multiple_capture_test() {
         let mut board = Board::from_mockup([
            ["  ", "  ", "  ", "WQ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "BP", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "BP", "  ", "  "],
            ["  ", "  ", "BP", "  ", "  ", "  ", "  ", "  "],
            ["  ", "BP", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "BP", "  "],
            ["  ", "  ", "  ", "BP", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "]]);
        println!("Before capturing:\n{}", board.repr());
        let pieces = MoveExecutor::get_pieces(&board, CheckersColor::White);
        let (_, cq) = MoveExecutor::get_capturing_pieces(&board, &pieces, CheckersColor::White);
        let poss_cap = MoveExecutor::get_possible_queen_captures(&board, cq, CheckersColor::White);
        board = MoveExecutor::execute_capture(&board, poss_cap.first().unwrap());
        println!("After capture:\n{}", board.repr());
        let comp_board = Board::from_mockup([
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "WQ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "BP", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "]]);
        assert_eq!(comp_board.get_board(), board.get_board());
    }

    #[test]
    fn get_all_pawn_moves_test() {
        println!("Checking basic pawns...");
        let board = Board::new(2).unwrap();
        let possible_moves: Vec<(usize, usize)> = MoveExecutor::get_all_moves(&board, CheckersColor::Black)
            .iter()
            .map(|m| m.end_pair())
            .unique()
            .collect();
        let res = vec![(2, 1), (2, 3), (2, 5), (2, 7)];
        assert_eq!(&res.len(), &possible_moves.len());
        for pos_mov in possible_moves {
            assert!(res.contains(&pos_mov));
        }

        println!("Checking queens...");
        let board = Board::from_mockup([
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "WQ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "WP", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "WP"],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "]]);
        println!("Base setup:\n{}", board.repr());
        let possible_moves: Vec<(usize, usize)> = MoveExecutor::get_all_moves(&board, CheckersColor::White)
            .iter()
            .map(|m| m.end_pair())
            .unique()
            .collect();
        let res = vec![(2, 1), (2, 3), (3, 6), (0, 3), (0, 5), (2, 5)];
        assert_eq!(&res.len(), &possible_moves.len());
        for pos_mov in possible_moves {
            assert!(res.contains(&pos_mov));
        }
    }

    #[test]
    fn promote_to_queen_test() {
        let mut board = Board::from_mockup([
            ["  ", "  ", "  ", "  ", "  ", "WP", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "WP", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "BP", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["BP", "  ", "BP", "  ", "  ", "  ", "  ", "  "]]);
        println!("Board setup:\n{}", board.repr());
        board = MoveExecutor::promote_to_queen(&board);
        println!("Post move:\n{}", board.repr());
        let board_cmp = Board::from_mockup([
            ["  ", "  ", "  ", "  ", "  ", "WQ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "WP", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "BP", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["BQ", "  ", "BQ", "  ", "  ", "  ", "  ", "  "]]);
        assert_eq!(board_cmp.get_board(), board.get_board());
    }
}