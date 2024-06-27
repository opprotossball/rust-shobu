use crate::shobu::Shobu;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_strings() {
        let position = "b w_www_______bb_b wwww________bbbb wwww________bbbb www__bw_____bbb_";
        let game = Shobu::from_string(position);
        assert_eq!(game.to_string(), position);
        assert_eq!(game.pieces[1][0], [7, 9, 10, 13]);
    }
}