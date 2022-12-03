use std::{fs, error::Error};

pub fn run(file_path: &str) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(file_path)?;

    let calories = calories(&contents);

    let solution = max_calories(&calories)?;
    let calories = solution.calories;
    let elf = solution.elf;
    println!("Max calories are: {calories}");
    println!("Its the elf number: {elf}");

    Ok(())
}

pub struct Solution {
    pub calories: i32,
    pub elf: i32,
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

pub fn max_calories(calories: &Vec<i32>) -> Result<Solution, Box<dyn Error>> {
    let max = calories.iter().max().unwrap();
    let elf = calories.iter().position(|element| element == max).unwrap();
    Ok(Solution { calories: *max, elf: elf as i32 })
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
}