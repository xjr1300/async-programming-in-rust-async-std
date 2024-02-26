use std::time::Duration;

use async_std::task;

fn main() {
    task::spawn(async {
        panic!("test");
    });

    task::block_on(async {
        task::sleep(Duration::from_millis(10_000)).await;
    });
}
