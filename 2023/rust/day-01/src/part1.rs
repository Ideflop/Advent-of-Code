pub fn process(input: &str) -> String {
    let mut result = 0;
    let lines: Vec<_> = input.split('\n').collect();
    let digit: Vec<Vec<u32>> = lines
        .iter()
        .map(|x| x.chars().filter_map(|d| d.to_digit(10)).collect())
        .collect();

    let mut values_of_each_lines: Vec<u32> = Vec::new();
    for a in digit {
        match a.len() {
            1 => values_of_each_lines.push((a[0].to_string() + &a[0].to_string()).parse().unwrap()),
            x if x > 1 => values_of_each_lines.push(
                (a[0].to_string() + &a[a.len() - 1].to_string())
                    .parse()
                    .unwrap(),
            ),
            _ => (),
        }
    }

    values_of_each_lines
        .iter()
        .for_each(|number| result += number);
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input));
    }
}
