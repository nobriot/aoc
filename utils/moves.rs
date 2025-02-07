use super::direction::Direction;

/// Move sequence, used in particular for 2-d navigation puzzles
///
pub struct Moves {
    pub moves: Vec<Direction>,
}

impl std::str::FromStr for Moves {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves: Vec<Direction> = s.chars().filter_map(|c| Direction::from_char(c)).collect();
        if moves.len() != s.len() {
            eprintln!("Invalid move sequence: {:?}", s);
            return Err(());
        }
        Ok(Self { moves })
    }
}
