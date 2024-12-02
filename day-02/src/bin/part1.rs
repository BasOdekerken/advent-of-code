fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let res: u32 = input
        .lines()
        .filter(|line| {
            let nums: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let is_ascending = nums.windows(2).all(|w| {w[0] < w[1] && w[0].abs_diff(w[1]) <= 3});
            let is_descending = nums.windows(2).all(|w| {w[0] > w[1] && w[0].abs_diff(w[1]) <= 3});

            is_ascending || is_descending
        })
        .count() as u32;
    return res;
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let result = part1("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        assert_eq!(result, 2);
    }
}