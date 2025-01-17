use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // create a vector to hold the list of tasks
    let mut tasks = Vec::new();

    // create task 1
    tasks.push(task::spawn(async {
        // perform some work here...
        println!("T1 Spawned a task sleeping for 10 seconds");
        sleep(Duration::from_secs(10)).await; // Wait for 10 seconds
        println!("T1 I woke up after 10 seconds");
    }));

    //create task2
    tasks.push(task::spawn(async {
        // perform some work here...
        println!("T2 Spawned a task sleeping for 2 seconds");
        sleep(Duration::from_secs(2)).await; // Wait for 2 seconds
        println!("T2 I woke up after 2 seconds");
    }));

    // Wait for all tasks to complete
    for task in tasks {
        task.await.unwrap();
    }
}
