use std::fs::OpenOptions;
use std::time::{Duration, Instant};
use nix::unistd::mkfifo;
use anyhow::Result;
use nix::sys::stat::Mode;
use tokio::io::{AsyncWriteExt};
use tokio::net::unix::pipe;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<()> {

    let dir = tempfile::Builder::new()
            .prefix("mitmproxy")
            .tempdir()?;
    let path = dbg!(dir.path().join("fifo"));

    dbg!(mkfifo(&path, Mode::S_IRWXU)?);

    // We cannot open for writing with tokio yet:
    // tokio uses non-blocking I/O, and that requires a writer to be present.
    // workaround: spawn writer first, then use blocking I/O in a thread
    // to determine when we can safely open.

    println!("spawning swift side...");
    Command::new("swift")
        .arg("../swift/read.swift")
        .arg(&path)
        .spawn()?;

    let p = path.clone();
    tokio::task::spawn_blocking(move || {
        println!("wait for write open");
        let t = Instant::now();
        OpenOptions::new().write(true).open(p).unwrap();
        println!("write open possible: {:?}", Instant::now() - t);
    }).await?;

    println!("opening...");
    let mut s = pipe::OpenOptions::new().open_sender(&path)?;

    loop {
        println!("writing...");
        s.write_u8(65).await?;
        tokio::time::sleep(Duration::from_millis(1500)).await;
    }
}
