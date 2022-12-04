use std::{error::Error, fs};

pub fn run(file_path: &str) -> Result<i32, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let total = calculate_total(&contents);
    let step2 = result_for_strategy(&contents);

    println!("The solution for step1 is: {total}");
    println!("The solution for step2 is: {step2}");

    Ok(total)
}

pub fn calculate_total(contents: &str) -> i32 {
    let mut total: i32 = 0;
    for line in contents.lines(){
        let round = Round::from_choices(line);
        let result = round.result();
        total += result;
    }
    total
}

pub fn result_for_strategy(contents: &str) -> i32 {
    let mut total: i32 = 0;
    for line in contents.lines(){
        let round = Round::from_strategy(line);
        let result = round.result();
        total += result;
    }
    total
}

#[derive(PartialEq, Clone, Copy)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

pub trait Beats {
    fn beats(&self) -> Self;
}

pub trait BeatenBy {
    fn beaten_by(&self) -> Self;
}

impl Beats for Choice {
    fn beats(&self) -> Self {
        match *self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }
}

impl BeatenBy for Choice {
    fn beaten_by(&self) -> Self {
        match *self {
            Choice::Scissors => Choice::Rock,
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
        }
    }
}

impl Choice {
    fn from(char: &str) -> Choice {
        match char {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => Choice::Rock
        }
    }

    fn leading_to(&self, char: &str) -> Choice {
        match char {
            "X" => self.beats(),
            "Y" => *self,
            "Z" => self.beaten_by(),
            _ => Choice::Rock
        }
    }

    fn value_of(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

struct Round {
    opponents_choice: Choice,
    my_choice: Choice,
}

impl Round {
    pub fn from_choices(line: &str) -> Round {
        let choices: Vec<&str> = line.split(" ").collect();
        Round { opponents_choice: Choice::from(choices[0]), my_choice: Choice::from(choices[1]) }
    }

    pub fn from_strategy(line: &str) -> Round {
        let input: Vec<&str> = line.split(" ").collect();
        let opponents_choice = Choice::from(input[0]);
        let my_choice = opponents_choice.leading_to(input[1]);
        Round { opponents_choice: opponents_choice, my_choice: my_choice }
    }
    
    pub fn result(&self) -> i32 {
        let mut score:i32;
        if self.opponents_choice.beats() == self.my_choice{
            score = 0;
        }
        else if self.my_choice.beats() == self.opponents_choice{
            score = 6;
        }
        else {
            score = 3;
        }
        score += self.my_choice.value_of();
        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_is_15() {
        let contents = "\
A Y
B X
C Z";
        let expected_result = 15;

        assert_eq!(expected_result, calculate_total(&contents));
    }

    #[test]
    fn strategy_is_12() {
        let contents = "\
A Y
B X
C Z";
        let expected_result = 12;

        assert_eq!(expected_result, result_for_strategy(&contents));
    }
}
