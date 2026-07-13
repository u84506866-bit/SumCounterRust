use std::io;

fn main() {
    let mut int1 = String::new();
    let mut int2 = String::new();

    io::stdin()
        .read_line(&mut int1)
        .expect("Failed to read line");

    io::stdin()
        .read_line(&mut int2)
        .expect("Failed to read line");

    let int1: i32 = int1.trim().parse().expect("Please Enter a Number!");
    let int2: i32 = int2.trim().parse().expect("Please Enter a Number!");
    let int3 = sum(int1, int2);
    println!("{int3}");
}

fn sum(inta: i32, intb: i32) -> i32 {
    let intc = inta + intb;
    intc
}
