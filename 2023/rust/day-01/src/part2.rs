struct Pair {
    letter: String,
    value: u32,
}

pub fn process(input: &str) -> String {
    let string_to_value: Vec<Pair> = vec![
        Pair {
            letter: "one".to_string(),
            value: 1,
        },
        Pair {
            letter: "two".to_string(),
            value: 2,
        },
        Pair {
            letter: "three".to_string(),
            value: 3,
        },
        Pair {
            letter: "four".to_string(),
            value: 4,
        },
        Pair {
            letter: "five".to_string(),
            value: 5,
        },
        Pair {
            letter: "six".to_string(),
            value: 6,
        },
        Pair {
            letter: "seven".to_string(),
            value: 7,
        },
        Pair {
            letter: "eight".to_string(),
            value: 8,
        },
        Pair {
            letter: "nine".to_string(),
            value: 9,
        },
    ];

    let mut result = 0;
    let lines: Vec<_> = input.split("\n").collect();
    let digit: Vec<Vec<u32>> = lines
        .iter()
        .filter_map(|x| {
            let mut temp: Vec<u32> = Vec::new();
            for i in 0..x.len() {
                let mut boolean = false;
                for a in string_to_value.iter() {
                    if !boolean {
                        if x[i..].starts_with(&a.letter) {
                            temp.push(a.value);
                            boolean = true;
                        }
                    }
                }
                if !boolean {
                    let first_letter = x.chars().nth(i).unwrap_or('a');
                    if first_letter.is_digit(10) {
                        temp.push(first_letter.to_digit(10).unwrap());
                    }
                }
            }

            return Some(temp);
        })
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
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input));
    }
}
