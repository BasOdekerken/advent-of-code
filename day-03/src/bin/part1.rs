use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let res = input.lines()
        .map(|line | {
            re.captures_iter(line)
                .map(|cap| {
                    let a: u32 = cap[1].parse().unwrap();
                    let b: u32 = cap[2].parse().unwrap();
                    a * b
                })
                .sum::<u32>()
        })
        .sum();

    return res
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let result = part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, 161);
    }
}