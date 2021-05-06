use std::io::Write;
use rss::Item;
use crate::error::EnsureError;
use crate::var::DAT_PATH;

pub fn format_item(item: &Item) -> Option<String> {
    Some(format!("{}|{}\n", item.pub_date()?, item.title()?))
}

pub fn append_read(items: Vec<Item>) -> Result<(), EnsureError> {
    let mut file = std::fs::OpenOptions::new().create(true).append(true).open(DAT_PATH.as_path())?;
    let contents = items.into_iter().fold(String::new(), |mut contents, item| {
        contents += &format_item(&item).unwrap();
        contents
    });
    write!(file, "{}", contents)?;
    Ok(())
}

pub fn filter_old(news: &mut Vec<Item>) -> Result<(), EnsureError> {
    let file = match std::fs::read_to_string(DAT_PATH.as_path()) {
        Ok(contents) => contents,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => { "".to_owned() },
        Err(e) => return Err(e.into())
    };
    news.retain(|item| {
        !file.contains(&format_item(item).unwrap())
    });
    Ok(())
}