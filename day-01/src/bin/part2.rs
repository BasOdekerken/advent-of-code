use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    // Not a great way to do this, but it works. HashMap would be better and then just key times both values.
    let (first_numbers, second_numbers): (Vec<u32>, Vec<u32>) = input
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

    let res  = first_numbers
        .iter()
        .map(|a| {
            let count = second_numbers.iter().filter(|&x| x == a).count() as u32;
            count * a
        })
        .sum();

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("3   4
4   3
2   5
1   3
3   9
3   3");
        assert_eq!(result, 31);
    }
}