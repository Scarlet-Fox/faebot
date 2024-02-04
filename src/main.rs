use poise::serenity_prelude as serenity;
use rand::seq::SliceRandom;
use rand::prelude::*;
use itertools::{Itertools, Position};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const PLUS: &str = "+";
const MINUS: &str = "-";
const BLANK: &str = "○";

struct  FudgeResult { total: i8, results: Vec<String> }
impl FudgeResult {
    fn merged_results(&self) -> String {
        let mut result_text = String::new();

        for result in self.results.iter().enumerate().with_position() {
            result_text += result.1.1;

            if result.0 != Position::Last && result.0 != Position::Only {
                result_text += " , "
            }
        }

        return result_text;
    }
    fn ladder_text(&self, stat_value: i8) -> String {
        let mut current_total = self.total + stat_value;
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

fn roll_fudge_dice<'a>() -> &'a str {
    let sides = [PLUS, MINUS, BLANK, PLUS, MINUS, BLANK];
    let mut rng = thread_rng();
    let result = sides.choose(&mut rng).expect("Failed to select an option.");
    return result;
}

fn roll_multiple_fudge<'a>(how_many: i8) -> FudgeResult {
    let mut current_total: i8 = 0;
    let mut results: Vec<String> = Vec::new();

    for number in 1..how_many+1 {
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

#[poise::command(slash_command, prefix_command)]
async fn fudge(
    ctx: Context<'_>,
    #[description = "Give a number to add to the roll"] stat_value: i8
) -> Result<(), Error> {
    let mut capped_stat_value= stat_value;
    if stat_value.saturating_sub(4) == i8::MIN {
        capped_stat_value = i8::MIN + 4;
    } else if stat_value.saturating_add(4) == i8::MAX {
        capped_stat_value = i8::MAX - 4;
    }
    let result = roll_multiple_fudge(4);
    let response = format!("Result: **{} ({} = {} + {})** [ {} ]",
                           result.ladder_text(capped_stat_value),
                           result.total + capped_stat_value, capped_stat_value, result.total,
                           result.merged_results());
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Missing discord token.");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions{
            commands: vec![fudge()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}

#[cfg(test)]
mod tests {
    use crate::roll_fudge_dice;

    #[test]
    fn test_roll() {
        let sides =  [crate::PLUS, crate::MINUS, crate::BLANK];
        assert!(sides.contains(&roll_fudge_dice()));
        assert!(sides.contains(&roll_fudge_dice()));
        assert!(sides.contains(&roll_fudge_dice()));
        assert!(sides.contains(&roll_fudge_dice()));
        assert!(sides.contains(&roll_fudge_dice()));
        assert!(sides.contains(&roll_fudge_dice()));
    }
}
