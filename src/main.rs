mod error;
mod fs;
mod var;

use error::EnsureError;
use var::SANITIZER;

use rss::Channel;
use structopt::StructOpt;

use crate::fs::filter_old;

#[derive(StructOpt, Debug)]
#[structopt(name = "ensure")]
enum Application {
    Check,
    Read {
        #[structopt(short)]
        unread: bool,
    }
}

fn main() {
    let app = Application::from_args();
    let news = fetch_news().unwrap();
    let mut unread = news.clone();
    filter_old(&mut unread.items).unwrap();
    match app {
        Application::Check => {
            std::process::exit(if unread.items.is_empty() { 0 } else { 1 });
        },
        Application::Read { unread: show_unread } => {
            for item in if show_unread { &news.items } else { &unread.items } {
                println!(
                    "{}\n{}\n",
                    item.title().unwrap(),
                    item.description().unwrap(),
                )
            }
            println!("Up to date.");
            fs::append_read(unread.items).unwrap();
        }
    }
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
