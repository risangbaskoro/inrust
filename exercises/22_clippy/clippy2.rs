fn main() {
    let mut res = 42;
    let option = Some(12);
    for x in option.into_iter() {
        res += x;
    }

    println!("{res}");
}
