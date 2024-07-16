pub mod game_moves;
pub mod game_structures;
use game_moves::get_moves;
use game_structures::*;
fn main() -> Result<(), GameErrors> {
    let gameboard = Board::Standard.init()?;
    for board_move in get_moves(gameboard, Piece::P(false)) {
        println!("{}", board_move)
    };
    Ok(())
}
