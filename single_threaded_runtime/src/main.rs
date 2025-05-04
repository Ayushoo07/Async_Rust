#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Even if we spawn multiple tasks, there is only one thread polling them.
    // They will be interleaved at .await points, but never run truly in parallel.
    
    let task1 = tokio::spawn(async {
        println!("Task1 started.");
        // Simulate async work, e.g. I/O, network request, etc.
        // Because we .await, we yield so other tasks can run in the meantime.
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        println!("Task1 completed.");
    });

    let task2 = tokio::spawn(async {
        println!("Task2 started.");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        println!("Task2 completed.");
    });

    // Join both tasks, waiting for them to finish
    let _ = tokio::join!(task1, task2);

    println!("All tasks done on a single-thread runtime.");
}

// What Happens Here?

//     Only one thread is used for all async tasks.

//     task1 sleeps for 2 seconds, task2 sleeps for 1 second, but whenever a task is waiting (sleeping, I/O), the runtime can switch to the other.

//     No parallelism: only concurrency via .await.