#[cfg(test)]
mod tests {
    use crate::shobu::{self, Shobu};

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
}