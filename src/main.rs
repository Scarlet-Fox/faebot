use poise::serenity_prelude as serenity;

mod utilities;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

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
    let result = utilities::roll_multiple_fudge(4);
    let response = format!("Result: **{} ({} = {} + {})** [ {} ]",
                           result.ladder_text(capped_stat_value),
                           result.total + capped_stat_value, capped_stat_value, result.total,
                           result.merged_results());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn xfudge(
    ctx: Context<'_>,
    #[description = "How many fudge dice should we roll?"] fudge_dice: u8,
    #[description = "Give a number to add to the roll"] stat_value: i8
) -> Result<(), Error> {
    let mut capped_dice_amount: i8 = fudge_dice as i8;
    if capped_dice_amount > 50 {
        capped_dice_amount = 50;
    }

    let mut capped_stat_value= stat_value;
    if stat_value.saturating_sub(capped_dice_amount) == i8::MIN {
        capped_stat_value = i8::MIN + capped_dice_amount;
    } else if stat_value.saturating_add(capped_dice_amount) == i8::MAX {
        capped_stat_value = i8::MAX - capped_dice_amount;
    }
    let result = utilities::roll_multiple_fudge(fudge_dice);
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
            commands: vec![fudge(), xfudge()],
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
