#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate xml5ever;
extern crate uuid;
extern crate reqwest;

pub mod entry;
pub mod feed;
pub mod parser;

pub use self::entry::Entry;
pub use self::feed::Feed;

#[cfg(test)]
mod tests {
    #[test]
    fn parse_rss_from_url() {
        if let Some(feed) = super::parser::from_url("https://thefullsnack.com/rss.xml")  {
            println!("{:?}", feed);
        }
    }

    #[test]
    fn parse_atom_from_url() {
        if let Some(feed) = super::parser::from_url("http://laptrinhcuocsong.com/feed.xml")  {
            println!("{:?}", feed);
        }
    }
}

