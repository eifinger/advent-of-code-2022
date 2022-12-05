use std::{error::Error, ops::Range};

pub fn solution1(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut total: i32 = 0;
    for line in contents.lines() {
        let pairs: Vec<&str> = line.split(',').collect();

        let (first_range, second_range) = parse_ranges(pairs);
        if ranges_have_subsets(first_range, second_range) {
            total += 1;
        }
    }
    Ok(total)
}

pub fn solution2(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut total: i32 = 0;
    for line in contents.lines() {
        let pairs: Vec<&str> = line.split(',').collect();

        let (first_range, second_range) = parse_ranges(pairs);
        for number in first_range {
            if second_range.contains(&number) {
                total += 1;
                break;
            }
        }
    }
    Ok(total)
}

fn parse_ranges(pairs: Vec<&str>) -> (Range<i32>, Range<i32>) {
    let first = pairs[0];
    let first_input: Vec<&str> = first.split('-').collect();
    let first_range = Range {
        start: first_input[0].parse::<i32>().unwrap(),
        end: first_input[1].parse::<i32>().unwrap() + 1,
    };

    let second = pairs[1];
    let second_input: Vec<&str> = second.split('-').collect();
    let second_range = Range {
        start: second_input[0].parse::<i32>().unwrap(),
        end: second_input[1].parse::<i32>().unwrap() + 1,
    };
    (first_range, second_range)
}

fn ranges_have_subsets(first: Range<i32>, second: Range<i32>) -> bool {
    if first.start >= second.start && first.end <= second.end {
        return true;
    }
    if second.start >= first.start && second.end <= first.end {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_is_2() {
        let contents = include_str!("fixtures/input_test.txt");
        let expected_result = 2;

        assert_eq!(expected_result, solution1(&contents).unwrap());
    }

    #[test]
    fn sample_is_4() {
        let contents = include_str!("fixtures/input_test.txt");
        let expected_result = 4;

        assert_eq!(expected_result, solution1(&contents).unwrap());
    }

    #[test]
    fn test_ranges_have_subsets() {
        let first = 4..6;
        let second = 6..6;

        let result = ranges_have_subsets(first, second);
        assert!(result);
    }
}
