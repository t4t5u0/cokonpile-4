use proconio::input;

// 三目並べ
// 2人でプレイする
// 


fn main() {
    println!("ゲームスタート");
    input!(
        x: u32,
        y: u32,
    );
    let board = Board::new(x, y);
    board.show();
}

// enum Result {
//     Win,
//     Lose,
//     Draw,
//     Continue,
// }

struct SanmokuNarabe{
    is_first: bool,
    turn: u32,
    goban: Board,
}

impl SanmokuNarabe{
    fn play(&self){

    }

    fn check(&self) -> bool {
        for i in 0..self.goban.x-2{
            for j in 0..self.goban.y-2{
                // 縦横斜めの判定
                // !配列外参照起こしそう
                if (self.goban.board[i as usize][j as usize] == self.goban.board[(i+1) as usize][j as usize] 
                    && self.goban.board[(i+1) as usize][j as usize] == self.goban.board[(i+2) as usize][j as usize])
                || (self.goban.board[i as usize][j as usize] == self.goban.board[i as usize][(j+1) as usize] 
                    && self.goban.board[i as usize][(j+1) as usize] == self.goban.board[i as usize][(j+2) as usize])
                || (self.goban.board[i as usize][j as usize] == self.goban.board[(i+1) as usize][(j+1) as usize] 
                    && self.goban.board[(i+1) as usize][(j+1) as usize] == self.goban.board[(i+2) as usize][(j+2) as usize])
                || (self.goban.board[(i+2) as usize][j as usize] == self.goban.board[(i+1) as usize][(j+1) as usize]
                    && self.goban.board[(i+1) as usize][(j+1) as usize] == self.goban.board[(i) as usize][(j+2) as usize])
                {
                    return true
                }
            }
        }
    return false
    }
    fn start_turn(&mut self) {
        println!("{}ターン目です。", self.turn);

        self.is_first = !self.is_first;
        self.turn += 1;
    }
}

struct Board{
    x: u32,
    y: u32,
    board: Vec<Vec<i32>>,
}

impl Board{
    fn new(x: u32, y: u32) -> Self{
        Board{
            x: x,
            y: y,
            board: vec![vec![0;y as usize];x as usize]}
    }

    fn show(&self){
        for item in self.board.iter(){
            for i in item{
                match i {
                    0 => print!(" "),
                    1 => print!("x"),
                    2 => print!("o"),
                    _ => print!("?")
                }
            }
            println!();
        }
    }
}