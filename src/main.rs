fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];

    let mut cont = 0;
    for n in slice {
        println!("slice[{cont}]: {n}");
        cont += 1;
    }
}
