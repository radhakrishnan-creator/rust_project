mod tic_tac_toe;

fn main() {
    let a = tic_tac_toe::TicTacToe {
        x_player: 1,
        o_player: 1,
    };
    println!("Hello, world! {}", 1);
    println!("Hello, world! {:#?}", a);
}
