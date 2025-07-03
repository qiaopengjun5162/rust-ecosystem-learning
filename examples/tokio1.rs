use std::thread;
use std::time::Duration;

use tokio::{fs, runtime::Builder, time::sleep};

fn main() {
    let handle = thread::spawn(|| {
        // execute future ?
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        rt.spawn(async {
            println!("Future 1 executed on thread");
            let content = fs::read_to_string("Cargo.toml").await.unwrap();
            println!("Content length: {}", content.len());
        });

        rt.spawn(async {
            println!("Future 2 executed on thread");
            let ret = expensive_blocking_task("Future 2".to_string());
            println!("Result: {ret}");
        });

        rt.block_on(async {
            sleep(Duration::from_millis(900)).await;
        });
    });

    handle.join().unwrap();
}

fn expensive_blocking_task(s: String) -> String {
    thread::sleep(Duration::from_millis(800));
    blake3::hash(s.as_bytes()).to_string()
}

// fn main() {
//     let handle = thread::spawn(|| {
//         // execute future ?
//         let rt = Builder::new_current_thread().enable_all().build().unwrap();
//         rt.block_on(async {
//             println!("Hello, world!");
//         });
//     });

//     handle.join().unwrap();
// }

// fn main() {
//     let handle = thread::spawn(|| println!("hello from the thread"));

//     handle.join().unwrap();
// }

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the first thread", i);
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread", i);
//     }

//     handle.join().unwrap();
// }

// #[tokio::main]
// async fn main() {
//     let a = 10;
//     let b = 20;
//     println!("{} + {} = {}", a, b, a + b);
// }
