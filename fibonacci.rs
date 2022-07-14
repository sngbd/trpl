use std::io;

fn main() {
    let mut num = String::new();

    println!("n: ");
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read input");

    let num: i32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    let mut prev = 0;
    let mut res = 1;
    let mut tmp;
    
    for _i in 1..num {
        tmp = res;
        res += prev;
        prev = tmp;
    }
    println!("n-th number: ");
    println!("{res}");
}
