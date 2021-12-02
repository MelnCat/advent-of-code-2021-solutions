fn main() {
    let input = include_str!("../input.txt");
    let numbers = input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let total = numbers.iter().enumerate().fold::<i32, _>(0, |l, (i, _)| {
        if i > 0 && get_triple_sum(&numbers, i) > get_triple_sum(&numbers, i - 1) {
            l + 1
        } else {
            l
        }
    });
    println!("{}", total);
}

fn get_triple_sum(numbers: &Vec<i32>, index: usize) -> i32 {
    if (index + 2) >= numbers.len() {
        return 0;
    }
    numbers[index] + numbers[index + 1] + numbers[index + 2]
}
