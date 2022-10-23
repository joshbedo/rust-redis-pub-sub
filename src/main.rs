
use tokio_stream::StreamExt;
use mini_redis::client;


async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "522".into()).await?;
    client.publish("numbers", "333".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "4".into()).await?;
    client.publish("numbers", "5".into()).await?;

    // client.set("joshbedo:votes:count", "1".into()).await?;
    // client.set("nominee:votes:count", "5".into()).await?;
    
    return Ok(());
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await?;
    let messages = subscriber
        .into_stream()
        .filter(|msg| match msg {
            Ok(msg) if msg.content.len() == 1 => true,
            _ => false,
        })
        .map(|msg| msg.unwrap().content)
        .take(3);

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }

    return Ok(());
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {

    tokio::spawn(async {
        publish().await
    });

    subscribe().await?;

    println!("DONE");

    // let up = '(';
    // let down = ')';
    // println!("x is {floating_point} {true_or}");

    return Ok(())
    // let mut tup: (i32, bool, char, &str) = (1, true, 's', "josh");
    // tup = (10, false, 'j', "josh");
    // let tup2: (i8, bool, char) = (1, true, 's');

    // println!("this is our {}", tup.3 );

}