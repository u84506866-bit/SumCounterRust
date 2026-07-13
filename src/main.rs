fn main() {
    let int1: i32 = 12;
    let int2: i32 = 24;
    let int3 = sum(int1, int2);
    println!("{int3}");
}

fn sum(inta: i32, intb: i32) -> i32 {
    let intc = inta + intb;
    intc
}
