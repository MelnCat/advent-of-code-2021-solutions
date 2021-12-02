fn main() {
    let input = include_str!("../input.txt");
    let numbers = input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let total = numbers.iter().enumerate().fold::<i32, _>(0, |l, (i, c)| {
        if i > 0 && c > &numbers[i - 1] {
            l + 1
        } else {
            l
        }
    });
    println!("{}", total);
}
