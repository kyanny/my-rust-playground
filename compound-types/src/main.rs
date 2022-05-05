fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    let tup2 = (500, 6.4, 1);
    let five_hundred = tup2.0;
    let six_point_four = tup2.1;
    let one = tup2.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);

    let a = [1, 2, 3, 4, 5];
    println!("{}", a[4]);

}
