#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use crate::{bot::ShobuBot, shobu::{self, Shobu, TILES, WHITE}, shobu_move::{internal_2_readable, readable_2_internal, Move}, zobrist::Zobrist};

    #[test]
    fn test_position_strings() {
        let position = "b w_www_______bb_b wwww________bbbb wwww________bbbb www__bw_____bbb_";
        let game = Shobu::from_string(position);
        assert_eq!(game.to_string(), position);
        assert_eq!(game.pieces[1][0], [7, 9, 10, 13]);
    }

    #[test]
    fn test_board_creation() {
        let position = "b wwww__________bb wwww___________b wwww___________b wwww___________b";
        let game = Shobu::from_string(position);
        assert_eq!(game.boards[0][27], shobu::BLACK);
        assert_eq!(game.boards[0][28], shobu::BLACK);
        assert_eq!(game.pieces[0][0], [27, 28, shobu::NOT_ON_BOARD, shobu::NOT_ON_BOARD]);
    }

    #[test]
    fn test_move_generation() {
        let positions = [
            "b wwww__________bb wwww___________b wwww___________b wwww___________b",
            "b wwww________b___ _________w__b___ wwww___b________ wwww___b________",
            "b wwww___________b _________w_____b wwww___b________ wwww___b________",
            "b wwww___________b wwww___________b wwww___________b wwww___________b",
            "b wwww________bbbb wwww________bbbb wwww________bbbb wwww________bbbb" 
        ];
        let n_moves = [26, 8, 14, 18, 174];
        for (i, position) in positions.into_iter().enumerate() {
            let game = Shobu::from_string(position);
            let moves = game.get_legal_moves();
            assert_eq!(moves.len(), n_moves[i])
        }
    }

    #[test]
    fn test_readable_2_internal() {
        for tile in (0..16).into_iter() {
            assert_eq!(readable_2_internal(tile), TILES[tile]);
        }
    }

    #[test]
    fn test_internal_2_readable() {
        for (i, tile) in TILES.into_iter().enumerate() {
            assert_eq!(internal_2_readable(tile), i);
        }
    }

    #[test]
    fn test_move_notation_black() {
        let encoded = "2Uw14h13";
        let active_player = -1;
        let mv = Move::from_string(encoded, active_player).unwrap();
        assert!(mv.double);
        assert_eq!(mv.board_1, 1);
        assert_eq!(mv.board_2, 0);
        assert_eq!(mv.direction, -6);
        assert_eq!(mv.from_1, 27);
        assert_eq!(mv.from_2, 26);
    }

    
    #[test]
    fn test_move_notation_white() {
        let encoded = "2DLw3f2";
        let active_player = 1;
        let mv = Move::from_string(encoded, active_player).unwrap();
        assert!(mv.double);
        assert_eq!(mv.board_1, 3);
        assert_eq!(mv.board_2, 0);
        assert_eq!(mv.direction, 5);
        assert_eq!(mv.from_1, 10);
        assert_eq!(mv.from_2, 9);
    }

    #[test]
    fn test_making_moves() {
        let moves = ["2Uw14h13"];
        let end_position = "w wwww_b______b_bb wwww__b_____bb_b wwww________bbbb wwww________bbbb";
        let mut game = Shobu::new();
        for mv_str in moves {
            let mv = Move::from_string(&mv_str, game.active_player).unwrap();
            game.make_move(&mv).unwrap();
        }
        assert_eq!(game.pieces[0][0], [25, 14, 27, 28]);
        assert_eq!(game.to_string(), end_position);
    }

    #[test]
    fn test_making_moves_whole_game() {
        let moves = ["2Uw14h13", "2DLw3f2", "2Ub14h13", "Db0h9", "2Ub15h12", "2DRw0h4", "Db7h6", "Db1h10", "Db6h10", "Lb2h13", "Uw14h10", "Rb1h14"];
        let end_position = "b ww_w__b_w__bb___ wwwwbb____b____b __ww_w______bbwb _ww_________w__w";
        let winner = WHITE;
        let mut game = Shobu::new();
        for mv_str in moves {
            let mv: Move = Move::from_string(&mv_str, game.active_player).unwrap();
            game.make_move(&mv).unwrap();
        }
        assert_eq!(game.to_string(), end_position);
        assert_eq!(game.winner, winner);
    }

    #[test]
    fn test_undo_move() {
        let n_moves = 16;
        let n_undos = 4;
        let seeds = [2137, 789, 8, 45, 123];
        for seed in seeds {
            let mut game = Shobu::new();
            let mut rand = StdRng::seed_from_u64(seed);
            let mut pos = "".to_string();
            for i in 0..n_moves {
                if i == n_moves - n_undos {
                    pos = game.to_string();
                }
                let moves = game.get_legal_moves();
                let _ = game.make_move(&moves[rand.gen_range(0..moves.len())]);
            }
            for _ in 0..n_undos {
                game.undo_move();
            }
            assert_eq!(game.to_string(), pos);
        }
    }

    #[test]
    fn test_pieces_undo() {
        let position = "b w_www_______bb_b wwww________bbbb wwww________bbbb www__bw_____bbb_";
        let mut game = Shobu::from_string(position);
        let n_moves = 40;
        let seed = 2137;
        let mut rand = StdRng::seed_from_u64(seed);
        for _ in 0..n_moves {
            let moves = game.get_legal_moves();
            let _ = game.make_move(&moves[rand.gen_range(0..moves.len())]);
        }
        for _ in 0..n_moves {
            game.undo_move();
        }
        assert_eq!(game.to_string(), position);
        assert_eq!(game.pieces[1][0], [7, 9, 10, 13]);
    }

    #[test]
    fn test_extended_move_to_string() {
        let encoded = "URb12h12";
        let game = Shobu::new();
        let mv = Move::from_string(&encoded, game.active_player).unwrap();
        let move_ext = game.validate_and_extend(&mv).unwrap();
        assert_eq!(move_ext.to_string(game.active_player), encoded);
    }

    #[test]
    fn test_win_in_1_move() {
        let positions = [
            "b w_b_____________ wb______________ wb______________ wb______________", 
            "w bw______________ bw______________ b_w_____________ bw______________"
        ];
        let winners = [-1, 1];
        for (winner, position) in std::iter::zip(winners, positions) {
            let mut game = Shobu::from_string(position);
            let mut bot = ShobuBot::new();
            let mv = bot.choose_move(&mut game);
            let mut validation_game = Shobu::from_string(position);
            validation_game.make_move(&mv).unwrap();
            assert_eq!(validation_game.winner, winner);
        }
    }

    #[test]
    fn test_zobrist_color_swap() {
        let zobrist = Zobrist::new();
        let position_1 = "b w_b_____________ wb______________ wb______________ w______________b";
        let position_2 = "b wb______________ w_b_____________ w______________b wb______________";
        let hash_1 = zobrist.get_hash(&Shobu::from_string(position_1), false, false);
        let hash_2 = zobrist.get_hash(&Shobu::from_string(position_2), true, false);
        assert_eq!(hash_1, hash_2);
    }

    #[test]
    fn test_zobrist_horizontal_swap() {
        let zobrist = Zobrist::new();
        let position_1 = "b w_b_____________ wb______________ wb______________ w______________b";
        let position_2 = "b _b_w____________ __bw____________ __bw____________ ___w________b___";
        let hash_1 = zobrist.get_hash(&Shobu::from_string(position_1), false, false);
        let hash_2 = zobrist.get_hash(&Shobu::from_string(position_2), false, true);
        assert_eq!(hash_1, hash_2);
    }

    #[test]
    fn test_zobrist_horizontal_and_color_swap() {
        let zobrist = Zobrist::new();
        let position_1 = "b w_b_____________ wb______________ wb______________ w______________b";
        let position_2 = "b __bw____________ _b_w____________ ___w________b___ __bw____________";
        let hash_1 = zobrist.get_hash(&Shobu::from_string(position_1), false, false);
        let hash_2 = zobrist.get_hash(&Shobu::from_string(position_2), true, true);
        assert_eq!(hash_1, hash_2);
    }

    // #[test]
    // fn gen_pos() {
    //     let g = Shobu::new();
    //     for mv in g.get_legal_moves() {
    //         let mut game = Shobu::new();
    //         game.make_move(&mv);
    //         println!("{}", game.to_string());
    //     }
    // }

}