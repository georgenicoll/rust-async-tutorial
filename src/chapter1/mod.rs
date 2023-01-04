use std::time::Duration;

use futures::executor::block_on;
use async_std;

async fn hello_world() {
    println!("hello, world!");
}

pub fn _1_3a() {
    let future = hello_world();
    block_on(future);
    println!("1.3 Done");
}

struct Song {
}

async fn learn_song() -> Song {
    println!("learning song...");
    async_std::task::sleep(Duration::from_millis(500)).await;
    Song {}
}

async fn sing_song(_song: Song) {
    println!("singing song...");
}

async fn dance() {
    println!("dancing...");
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn make_a_song_and_dance() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

pub fn _1_3b() {
    //sequenced
    // let song = block_on(learn_song());
    // block_on(sing_song(song));
    // block_on(dance());
    //some parallelisation
    block_on(make_a_song_and_dance());
}