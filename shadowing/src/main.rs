fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is :{}", x);

    let spaces = "  ";
    println!("spaces=[{}]", spaces);
    let spaces = spaces.len();
    println!("spaces=[{}]", spaces);

    let mut spaces = "  ";
    spaces = spaces.len();
}
