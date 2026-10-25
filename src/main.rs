use crate::counter::CounterHandle;

mod actor;
mod counter;

#[tokio::main]
async fn main() {
    println!("Hello world");

    let counter1 = CounterHandle::new();

    let mut current_count = counter1.get_count().await;
    println!("Current count: {}", current_count);

    counter1.set_count(42).await;

    current_count = counter1.get_count().await;
    println!("Current count: {}", current_count);

    counter1.reset_count().await;

    current_count = counter1.get_count().await;
    println!("Current state after reset: {}", current_count);

    let counter2 = counter1.clone();

    current_count = counter2.get_count().await;
    println!("Current count: {}", current_count);

    counter2.set_count(69).await;

    current_count = counter2.get_count().await;
    println!("Current count: {}", current_count);

    counter2.reset_count().await;

    current_count = counter2.get_count().await;
    println!("Current state after reset: {}", current_count);
}
