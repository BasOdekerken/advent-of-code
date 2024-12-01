fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let (mut first_numbers, mut second_numbers): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut numbers  = line
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap());
            (
                numbers.next().unwrap(),
                numbers.next().unwrap(),
            )
        })
        .unzip();

    first_numbers.sort();
    second_numbers.sort();

    let res = first_numbers
        .iter()
        .zip(second_numbers.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>();
    return res;
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let result = part1("3   4
4   3
2   5
1   3
3   9
3   3");
        assert_eq!(result, 11);
    }
}