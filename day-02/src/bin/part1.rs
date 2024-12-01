fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
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