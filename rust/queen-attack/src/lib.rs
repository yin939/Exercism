#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank >= 8 || file < 0 || file >= 8 {
            return None;
        }
        Some(ChessPosition { rank, file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.rank == other.position.rank
            || self.position.file == other.position.file
            || is_diagonal(&self.position, &other.position)
        {
            return true;
        }

        false
    }
}

fn is_diagonal(p1: &ChessPosition, p2: &ChessPosition) -> bool {
    (p1.rank - p2.rank).abs() == (p1.file - p2.file).abs()
}
