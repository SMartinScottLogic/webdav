use clap::Parser;
use rustydav::{client, prelude::*};

#[derive(Parser, Debug)]
#[command(
    version,
    about("Perform a few tests against a WebDAV server"),
)]
struct Args {
    /// Server
    #[arg(short, long, default_value = "http://localhost:8080")]
    server: String,

    #[arg(short, long, default_value = "")]
    username: String,

    #[arg(short, long, default_value = "")]
    password: String,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let path = |p: &str| {
        format!("{}/{}", args.server, p)
    };
    let dav = client::Client::init(&args.username, &args.password);
    let r = dav.list(&path("/"), "1")?;
    println!("Initial listing: {:?}", r.text());
    let r = dav.put("hello world", &path("/test.txt"))?;
    println!("Write 'hello world' to test.txt: {:?}", r.text());
    let r = dav.list(&path("/"), "1")?;
    println!("Listing: {:?}", r.text());
    let r = dav.get(&path("/test.txt"))?;
    println!("Content of test.txt: {:?}", r.text());
    let r = dav.put("goodbye", &path("/test.txt"))?;
    println!("Overwrite test.txt: {:?}", r.text());
    let r = dav.get(&path("/test.txt"))?;
    println!("Content of test.txt: {:?}", r.text());
    let r = dav.delete(&path("/test.txt"))?;
    println!("Delete test.txt: {:?}", r.text());
    let r = dav.get(&path("/test.txt"))?;
    println!("Final listing: {:?}", r.text());
    Ok(())
}
