use std::io::{Write, stdout};
use proconio::input;

// 三目並べ
// 2人でプレイする
// 


fn main() {
    print!("碁盤の大きさを指定してください: ");
    stdout().flush().unwrap();
    input!(
        x: u32,
        y: u32,
    );
    println!();
    println!("ゲームスタート");
    let mut sanmoku = SanmokuNarabe::new(Board::new(x, y));
    sanmoku.play();
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
    fn new(board: Board) -> Self{
        SanmokuNarabe{
            is_first : true,
            turn: 0,
            goban: board,
        }
    }


    fn play(&mut self){
        for turn in 1..=self.goban.x*self.goban.y{
            self.turn = turn
            
        }
    }

    fn check(&self) -> bool {
        for i in 0..self.goban.x-2{
            for j in 0..self.goban.y-2{
                // 縦横斜めの判定
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
        // 下２段は別に処理する
        for i in 0..self.goban.x-2{
            for j in self.goban.y..self.goban.y{
                if self.goban.board[i as usize][j as usize] == self.goban.board[(i+1) as usize][j as usize] 
                    && self.goban.board[(i+1) as usize][j as usize] == self.goban.board[(i+2) as usize][j as usize]
                {
                    return true
                }
            }
        }
        return false
    }
    fn start_turn(&mut self) {
        println!("{}ターン目です。", self.turn);
        println!("{}のターンです", if self.is_first {"先攻"} else {"後攻"});
        print!("マスを指定してください(x y): ");
        input!{
            x: u32,
            y: u32,
        };
        let result = self.check_inside(x, y);
        if !result{
            println!("碁盤の外にどうやって石を置くんですか？")
        }
        self.is_first = !self.is_first;
        self.turn += 1;
    }

    fn check_inside(&self, x:u32, y:u32) -> bool{
        if self.goban.x >= x && self.goban.y >= y{
            return true
        }
        return false
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
            board: vec![vec![0;y as usize];x as usize]
        }
    }

    fn show(&self){
        for i in 0..self.board.len(){
            print!(" {}",i);
        }
        println!();
        for (index, item) in self.board.iter().enumerate(){
            print!("{} ", index);
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