use std::{fs, error::Error};

pub fn run(file_path: &str) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(file_path)?;

    let all_calories = calories(&contents);

    let solution = max_calories(&all_calories)?;
    let calories = solution.calories;
    let elf = solution.elf;
    let number_of_elves = solution.total;
    println!("Max calories are: {calories}");
    println!("Its the elf number: {elf}");
    println!("In total there are: {number_of_elves} elves");

    let top_three = top_three(&all_calories)?;
    let total = top_three.total;

    println!("The top three elves are carrying: {total} calories");

    Ok(())
}

pub struct MaxCalories {
    pub calories: i32,
    pub elf: i32,
    pub total: i32,
}

pub struct TopThreeCalories {
    pub one: i32,
    pub two: i32,
    pub three: i32,
    pub total: i32,
}

pub fn calories(contents: &str) -> Vec<i32> {
    let mut results = Vec::new();
    let mut elf: i32 = 0;
    for line in contents.lines() {
        if line.is_empty() {
            results.push(elf);
            elf = 0;
        }else{
            let calory = line.parse::<i32>().unwrap();
        elf += calory;
        }
    }
    results.push(elf);

    results
}

pub fn max_calories(calories: &Vec<i32>) -> Result<MaxCalories, Box<dyn Error>> {
    let max = calories.iter().max().unwrap();
    let elf = calories.iter().position(|element| element == max).unwrap();
    let total = calories.len();
    Ok(MaxCalories { calories: *max, elf: elf as i32, total: total as i32})
}

pub fn top_three(calories: &Vec<i32>) -> Result<TopThreeCalories, Box<dyn Error>> {
    let mut sorted = calories.clone();
    sorted.sort_unstable();
    sorted.reverse();
    let top_three = &sorted[0..3];
    let total: i32 = top_three.iter().sum();
    Ok(TopThreeCalories { one: top_three[0], two: top_three[1], three: top_three[2], total: total})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calories_are_calculated() {
        let contents = "\
22243
11899

1696
2595

6513
1151";

        assert_eq!(vec![34142,4291,7664], calories(contents));
    }

    #[test]
    fn max_calories_is_calculated() {
        let calories = vec![34142,4291,7664];
        let expected_max_calories = 34142;
        let expected_elf = 0;

        let solution = max_calories(&calories).unwrap();

        assert_eq!(&expected_max_calories, &solution.calories);
        assert_eq!(&expected_elf, &solution.elf);
    }

    #[test]
    fn top_three_is_calculated() {
        let calories = vec![34142,4291,7664,354,789];
        let expected_first = 34142;
        let expected_second = 7664;
        let expected_third = 4291;
        let expected_total = 46097;

        let top_three = top_three(&calories).unwrap();

        assert_eq!(&expected_first, &top_three.one);
        assert_eq!(&expected_second, &top_three.two);
        assert_eq!(&expected_third, &top_three.three);
        assert_eq!(&expected_total, &top_three.total);
    }
}