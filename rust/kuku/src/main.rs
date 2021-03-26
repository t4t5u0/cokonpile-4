fn main() {
    println!("九九の表です");
    solve();
}

fn solve() {
    for i in 1..=9{
        for j in 1..=9{
            print!("{:2} ", i*j);
        }
        println!();
    }
}
