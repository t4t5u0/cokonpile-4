use std::io::{Write, stdout};
use proconio::input;

// 三目並べ
// 2人でプレイする
// 


fn main() {
    print!("碁盤の大きさを指定してください: ");
    stdout().flush().unwrap();
    input!(
        x: u8,
        y: u8,
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
    turn: u8,
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
            self.turn = turn;
            self.start_turn();
            self.goban.show();
            if self.check(){
                println!("{} の勝利です", if self.is_first {"先攻"} else {"後攻"});
                return
            }
            self.end_turn();
        }
        println!("draw");
    }

    fn check(&self) -> bool {
        for i in 0..(self.goban.x-2) as usize {
            for j in 0..(self.goban.y-2) as usize{
                // 縦横斜めの判定

                let upper_left_is_stone = self.goban.board[i][j] == 1 || self.goban.board[i][j] == 2;
                let upper_right_is_stone = self.goban.board[(i+2)][j] == 1 || self.goban.board[(i+2)][j] == 2;
                let check_horizontal = self.goban.board[i][j] == self.goban.board[(i+1)][j] && self.goban.board[(i+1)][j] == self.goban.board[(i+2)][j];
                let check_vertical = self.goban.board[i][j] == self.goban.board[i][(j+1)] && self.goban.board[i][(j+1)] == self.goban.board[i][(j+2)];
                let check_left_diagnol = self.goban.board[i][j] == self.goban.board[(i+1)][(j+1)] && self.goban.board[(i+1)][(j+1)] == self.goban.board[(i+2)][(j+2)];
                let check_right_diagnol  = self.goban.board[(i+2)][j] == self.goban.board[(i+1)][(j+1)] && self.goban.board[(i+1)][(j+1)] == self.goban.board[(i)][(j+2)];
                // println!("{} {} {} {} {} {} ", upper_left_is_stone, upper_right_is_stone, check_horizontal, check_vertical, check_left_diagnol, check_right_diagnol);
                if (upper_left_is_stone && (check_horizontal || check_vertical || check_left_diagnol)) || (upper_right_is_stone && check_right_diagnol) {
                    return true
                }
            }
        }
        // 下２段は別に処理する
        for i in 0..(self.goban.x-2) as usize {
            for j in (self.goban.y as usize)..(self.goban.y as usize) {
                let upper_left_is_stone = self.goban.board[i][j] == 1 || self.goban.board[i][j] == 2;
                let check_horizontal = self.goban.board[i][j] == self.goban.board[(i+1)][j] && self.goban.board[(i+1)][j] == self.goban.board[(i+2)][j];
                if upper_left_is_stone && check_horizontal {
                    return true
                }
            }
        }
        for j in ((self.goban.y-2) as usize)..(self.goban.y as usize){
            let x :usize = (self.goban.x - 3).into();
            let is_stone = self.goban.board[x][j] == 1 || self.goban.board[x][j] == 2;
            let check_vertical = self.goban.board[x][j] == self.goban.board[(x+1)][j] && self.goban.board[(x+1)][j] == self.goban.board[(x+2)][j]
            if is_stone && check_vertical {
                return true
            } 
        }
        return false
    }
    fn start_turn(&mut self) {
        println!("{}ターン目です。", self.turn);
        println!("{}のターンです", if self.is_first {"先攻"} else {"後攻"});
        print!("マスを指定してください(x y): ");
        stdout().flush().unwrap();
        input!{
            _x: u8,
            _y: u8,
        };
        let mut x = _x;
        let mut y = _y;
        // let inside_flag = self.check_inside(x, y);
        let (mut inside_flag, mut duplicate_flag) = self.check_enable_put(x, y);
        println!("{} {}",inside_flag, duplicate_flag);
        while !(inside_flag && duplicate_flag){
            if !inside_flag{
                println!("碁盤の外にどうやって石を置くんですか？")
            }
            if !duplicate_flag{
                println!("碁石の上に碁石を重ねるゲームではありません")
            }
            print!("マスを指定してください(x y): ");
            stdout().flush().unwrap();
            input!{
                _x: u8,
                _y: u8,
            };
            x = _x; y = _y;
            let( _inside_flag, _duplicate_flag) = self.check_enable_put(x, y);
            inside_flag = _inside_flag;
            duplicate_flag = _duplicate_flag;
            // self.goban.put(x, y, if self.is_first {1} else {2});
        }
        self.goban.put(x, y, if self.is_first {1} else {2});
    }
    fn end_turn(& mut self){
        self.is_first = !self.is_first;
        self.turn += 1;
    }

    // fn check_inside(&self, x:u8, y:u8) -> bool{
    //     if self.goban.x < x || self.goban.y < y{
    //         return false;
    //     }
    //     return true;
    // }

    fn check_enable_put(&self, x:u8, y:u8) -> (bool, bool){
        let mut flag1 = true;
        let mut flag2 = true;
        if self.goban.x <= x || self.goban.y <= y{
            flag1 = false;
        }
        if flag1 && self.goban.board[x as usize][y as usize] != 0{
            flag2 = false
        }
        return (flag1, flag2);
    }
}

struct Board{
    x: u8,
    y: u8,
    board: Vec<Vec<u8>>,
}

impl Board{
    fn new(x: u8, y: u8) -> Self{
        Board{
            x: x,
            y: y,
            board: vec![vec![0;y as usize];x as usize]
        }
    }

    fn put(&mut self, _x:u8, _y:u8, n:u8){
        self.board[_x as usize][_y as usize] = n;
    } 

    fn show(&self){
        for i in 0..self.board.len(){
            print!(" {}",i);
            stdout().flush().unwrap();
        }
        println!();
        for (index, item) in self.board.iter().enumerate(){
            print!("{} ", index);
            stdout().flush().unwrap();
            for i in item{
                match i {
                    0 => print!(" "),
                    1 => print!("o"),
                    2 => print!("x"),
                    _ => print!("?")
                }
            }
            stdout().flush().unwrap();
            println!();
        }
    }
}