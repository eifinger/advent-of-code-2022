use std::{error::Error};

pub fn solution1(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut result: i32 = 0;
    for line in contents.lines(){
        let (comp1, comp2) = compartments(line);
        let duplicate = find_duplicate(&comp1, &comp2)?;
        result += duplicate.priority();
    }
    Ok(result)
}

fn compartments(contents: &str) -> (&str, &str){
    contents.split_at(contents.len() / 2)
}

fn find_duplicate(first: &str, second: &str) -> Result<char, &'static str> {
    for char in first.chars() {
        if second.contains(char){
            let result = char;
            return Ok(result)
        }
    }
    Err("Could not find a duplicate")
}

pub fn solution2(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut result: i32 = 0;
    let mut group: Vec<&str> = Vec::new();
    let mut groups: Vec<Vec<&str>> = Vec::new();
    for line in contents.lines(){
        group.push(line);
        if group.len() == 3{
            groups.push(group.clone());
            group = Vec::new();
        }
    }

    for group in groups{
        let common = find_common(group[0], group[1], group[2])?;
        result += common.priority();
    }
    Ok(result)
}

fn find_common(first: &str, second: &str, third: &str) -> Result<char, &'static str> {
    for char in first.chars() {
        if second.contains(char){
            if third.contains(char){
                let result = char;
                return Ok(result)
            }
        }
    }
    Err("Could not find a duplicate")
}

trait Priority {
    fn priority(&self) -> i32;
}

impl Priority for char {
    fn priority(&self) -> i32 {
        (*self as i32) - if char::is_ascii_lowercase(&self) {96} else {38}
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_is_157() {
        let contents = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let expected_result = 157;

        assert_eq!(expected_result, solution1(&contents).unwrap());
    }

    #[test]
    fn compartements_cuts_in_half() {
        let contents = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let expected_compartment_1 = "vJrwpWtwJgWr";
        let expected_compartment_2 = "hcsFMMfFFhFp";

        let (comp1, comp2) = compartments(&contents);

        assert_eq!(expected_compartment_1, comp1);
        assert_eq!(expected_compartment_2, comp2);
    }

    #[test]
    fn finds_duplicate() {
        let comp1 = "vJrwpWtwJgWr";
        let comp2 = "hcsFMMfFFhFp";
        let expected_duplicate = 'p';

        let duplicate = find_duplicate(&comp1, &comp2).unwrap();

        assert_eq!(expected_duplicate, duplicate);
    }

    #[test]
    fn priority() {
        assert_eq!(1, 'a'.priority());
        assert_eq!(27, 'A'.priority());
        assert_eq!(16, 'p'.priority());
    }

    #[test]
    fn finds_common() {
        let first = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let second = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let third = "PmmdzqPrVvPwwTWBwg";

        let common = find_common(first, second, third).unwrap();

        assert_eq!('r', common);
    }

    #[test]
    fn sample_is_70() {
        let contents = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let expected_result = 70;

        assert_eq!(expected_result, solution2(&contents).unwrap());
    }
}
