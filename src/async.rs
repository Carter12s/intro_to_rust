#![allow(dead_code)]
/// How do we unlock even more power of this thread safety?
/// async and await!

async fn sqr(input: u64) -> u64 {
    input * input
}

fn use_sqr() {
    // This doesn't actually compute the result
    // This only generates a future!
    // The future is async state machine that will eventually produce the result
    let value = sqr(2);

    // Until we actually "run" the future nothing happens
    let _result = futures::executor::block_on(value);
}

async fn use_sqr_in_async() {
    // Within an async function we can "block" on another future with .await
    let _result = sqr(4).await;

    // I can also await many futures concurrently with join operations
    let mut futures = vec![];
    for i in 0..100 {
        futures.push(sqr(i));
    }
    let x = futures::future::join_all(futures).await;
    println!("{x:?}");
}

async fn select_race() {
    let f1 = async {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        1
    };

    let f2 = async {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        2
    };

    tokio::select! {
        res1 = f1 => {
        println!("got {res1}");
        }
        res2 = f2 => {
        println!("got {res2}");
        }
    };
}

#[tokio::main]
async fn main() {
    use_sqr_in_async().await;

    select_race().await;
}
