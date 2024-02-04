use itertools::{Itertools, Position};
use rand::prelude::SliceRandom;
use rand::thread_rng;

const PLUS: &str = "+";
const MINUS: &str = "-";
const BLANK: &str = "○";

pub struct  FudgeResult { pub(crate) total: i8, results: Vec<String> }
impl FudgeResult {
    pub(crate) fn merged_results(&self) -> String {
        let mut result_text = String::new();

        for result in self.results.iter().enumerate().with_position() {
            result_text += result.1.1;

            if result.0 != Position::Last && result.0 != Position::Only {
                result_text += " , "
            }
        }

        return result_text;
    }
    pub(crate) fn ladder_text(&self, stat_value: i8) -> String {
        let current_total = self.total + stat_value;
        let ladder_text = match current_total {
            8 => "Legendary",
            7 => "Epic",
            6 => "Fantastic",
            5 => "Superb",
            4 => "Great",
            3 => "Good",
            2 => "Fair",
            1 => "Average",
            0 => "Mediocre",
            -1 => "Poor",
            -2 => "Terrible",
            _ => "Holy !@#$%"
        };
        return ladder_text.to_string();
    }
}

pub(crate) fn roll_fudge_dice<'a>() -> &'a str {
    let sides = [PLUS, MINUS, BLANK, PLUS, MINUS, BLANK];
    let mut rng = thread_rng();
    let result = sides.choose(&mut rng).expect("Failed to select an option.");
    return result;
}

pub(crate) fn roll_multiple_fudge<'a>(how_many: u8) -> FudgeResult {
    let mut current_total: i8 = 0;
    let mut results: Vec<String> = Vec::new();

    for _number in 1..how_many+1 {
        let current_result = roll_fudge_dice();

        match current_result {
            PLUS => {
                current_total += 1;
            }
            MINUS => {
                current_total -= 1;
            }
            _ => {}
        }
        results.push(String::from(current_result));
    }

    return FudgeResult {
        total: current_total,
        results
    };
}

#[cfg(test)]
mod tests {
    use crate::utilities::{roll_fudge_dice, PLUS, MINUS, BLANK};

    #[test]
    fn test_roll() {
        let sides =  [PLUS, MINUS, BLANK];
        assert!(sides.contains(&roll_fudge_dice()));
        assert!(sides.contains(&roll_fudge_dice()));
        assert!(sides.contains(&roll_fudge_dice()));
        assert!(sides.contains(&roll_fudge_dice()));
        assert!(sides.contains(&roll_fudge_dice()));
        assert!(sides.contains(&roll_fudge_dice()));
    }
}
