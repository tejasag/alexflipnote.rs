use alexflipnote::{parse_filter, parse_icon, AlexClient, ColorAPIResponse, CowValue};
use dotenv::dotenv;
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::{
        macros::{command, group},
        Args, CommandResult, StandardFramework,
    },
    http::AttachmentType,
    model::{channel::Message, gateway::Ready},
};
use std::env;

#[group]
#[commands(color, drake, achievement, amiajoke, colorify, filter)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected and ready to go!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token!");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while starting the client:\n{:?}", why);
    }
}

async fn get_color(hex: &str) -> ColorAPIResponse {
    let client = AlexClient::new("yer key here uwu");
    client.color(&hex).await.unwrap()
}

async fn get_drake(top: &str, bottom: &str) -> bytes::Bytes {
    let client = AlexClient::new("yer key here uwu");
    client.drake(&top, &bottom).await.unwrap()
}

async fn get_achievement(text: &str, icon: &i32) -> bytes::Bytes {
    let client = AlexClient::new("yer key here uwu");
    client.achievement(&text, &icon).await.unwrap()
}

#[command]
async fn color(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let hex = args.single::<String>()?;
    let color_info = get_color(&hex).await;
    msg.reply(ctx, color_info.image_gradient).await?;
    Ok(())
}

#[command]
async fn drake(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let top = args.single::<String>()?;
    let bottom = args.single::<String>()?;
    let image = get_drake(&top, &bottom).await;
    msg.channel_id
        .send_message(ctx, |m| {
            m.add_file(AttachmentType::Bytes {
                //   data: Cow::from((&*image) as &[u8]),
                data: image.as_cow(),
                filename: "drake.png".to_string(),
            });
            m
        })
        .await?;
    Ok(())
}

#[command]
async fn achievement(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let icon = parse_icon(&mut *args.single::<String>()?) as i32;
    let text = args.rest();
    let image = get_achievement(&text, &icon).await;
    msg.channel_id
        .send_message(ctx, |m| {
            m.add_file(AttachmentType::Bytes {
                //   data: Cow::from((&*image) as &[u8]),
                data: image.as_cow(),
                filename: "achievement.png".to_string(),
            });
            m
        })
        .await?;
    Ok(())
}

#[command]
async fn filter(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut arg = args.single::<String>()?;
    let name = parse_filter(&mut arg);
    let text = args.rest();
    let client = AlexClient::new("yer key here uwu");
    let data = client.filter(name, &text).await.unwrap();
    msg.channel_id
        .send_message(ctx, |m| {
            m.add_file(AttachmentType::Bytes {
                //   data: Cow::from((&*image) as &[u8]),
                data: data.as_cow(),
                filename: "filter.png".to_string(),
            });
            m
        })
        .await?;
    Ok(())
}

#[command]
async fn amiajoke(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let url = args.raw().collect::<Vec<&str>>().join("");
    let client = AlexClient::new("yer key here uwu");
    let data = client.amiajoke(&url).await.unwrap();
    msg.channel_id
        .send_message(ctx, |m| {
            m.add_file(AttachmentType::Bytes {
                data: data.as_cow(),
                filename: "amiajoke.png".to_string(),
            });
            m
        })
        .await?;
    Ok(())
}

#[command]
async fn colorify(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = args.single::<String>()?;
    let color = args.single::<String>()?;
    let background = args.single::<String>()?;
    let client = AlexClient::new("yer key here uwu");
    let data = client
        .colorify(&url, &color, &background)
        .await
        .expect("Could not parse.");
    msg.channel_id
        .send_message(ctx, |m| {
            m.add_file(AttachmentType::Bytes {
                data: data.as_cow(),
                filename: "colorify.png".to_string(),
            });
            m
        })
        .await?;
    Ok(())
}
