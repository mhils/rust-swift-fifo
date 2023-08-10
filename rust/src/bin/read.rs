use nix::unistd::mkfifo;
use anyhow::Result;
use nix::sys::stat::Mode;
use tokio::io::{AsyncReadExt};
use tokio::net::unix::pipe;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<()> {

    let dir = tempfile::Builder::new()
            .prefix("mitmproxy")
            .tempdir()?;
    let path = dbg!(dir.path().join("fifo"));

    dbg!(mkfifo(&path, Mode::S_IRWXU)?);

    // we can open for read instantly...
    println!("opening...");
    let mut s = pipe::OpenOptions::new().open_receiver(&path)?;

    // ... and then later spawn the writer.
    println!("spawning swift side...");
    Command::new("swift")
        .arg("../swift/write.swift")
        .arg(path)
        .spawn()?;

    loop {
        println!(".readable().await...");
        s.readable().await?;
        dbg!(s.read_u8().await?);
    }
}
