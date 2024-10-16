fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    println!("{} is largest and {} is smallest", input.iter().max().unwrap(), input.iter().min().unwrap());
}
