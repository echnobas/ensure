mod error;
mod fs;
mod var;

use error::EnsureError;
use var::SANITIZER;

use rss::Channel;
use structopt::StructOpt;

use crate::fs::filter_old;

extern "C" {
    fn getppid() -> i32;
    fn geteuid() -> u32;
}


#[derive(StructOpt, Debug)]
#[structopt(name = "ensure")]
enum Application {
    Check,
    Read {
        #[structopt(short)]
        unread: bool,
    }
}

fn is_pacman() -> std::io::Result<bool> {
    let process = std::process::Command::new("ps").args(&["-p", unsafe { &getppid().to_string() }, "-o", "comm="]).output()?.stdout;
    let process = std::str::from_utf8(&process).unwrap();
    Ok(process.trim() == "pacman")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if unsafe { geteuid() } != 0 {
        eprintln!("ensure must be run as root!");
        std::process::exit(1);
    }
    let app = Application::from_args();
    let news = fetch_news()?;
    let mut unread = news.clone();
    filter_old(&mut unread.items)?;
    let read = unread.items.is_empty();
    match app {
        Application::Check => {
            if is_pacman()? && !read {
                println!("  :: Ensure ::    You have unread news! Read it with `ensure read`");
            } else if !read {
                println!("You have unread news! Read it with `ensure read`")
            } else {
                println!("Up to date.")
            }
            std::process::exit(if read { 0 } else { 1 });
        },
        Application::Read { unread: show_unread } => {
            if !read {
                for item in if show_unread { &news.items } else { &unread.items } {
                    println!(
                        "{} <> {}\n{}\n",
                        item.pub_date().unwrap(),
                        item.title().unwrap(),
                        item.description().unwrap(),
                    )
                }
                fs::append_read(unread.items).unwrap();
            } else {
                println!("Up to date.");
            }
        }
    }
    Ok(())
}

fn fetch_news() -> Result<Channel, EnsureError> {
    let feed = ureq::get("https://www.archlinux.org/feeds/news/")
        .call()?
        .into_string()?;
    let mut channel = Channel::read_from(feed.as_bytes())?;
    for item in &mut channel.items {
        item.set_description(Some(
            SANITIZER
                .replace_all(item.description().ok_or(EnsureError::Malformed)?, "")
                .to_string(),
        ))
    }
    Ok(channel)
}
