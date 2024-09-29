use tokio::signal;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let mut tasks = vec![];

    for i in 1..=20 {
        let token = token.clone();
        let task = tokio::spawn(async move {
            // Уводим вложенные задачи на новую очередь
            some_work(i);
            // Асинхронно ждем отмены токена
            token.cancelled().await;
            println!("task {i} shutdown gracefully");
        });
        tasks.push(task);
    }

    tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();
        token.cancel();
    });

    for task in tasks {
        task.await.unwrap();
    }
}

fn some_work(i: i32) {
    std::thread::spawn(move || {
        loop {
            println!("some work from task {i}");
            std::thread::sleep(std::time::Duration::from_millis(250 * i as u64));
        }
    });
}
