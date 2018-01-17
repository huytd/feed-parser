# feed-parser

This is a fork from [kumabook/feed-rs](https://github.com/kumabook/feed-rs), added an ability to parse a feed from URL.

Usage:

```rust
extern crate feed_parser;

use feed_parser::parser;

fn main() {
  if let Some(feed) = parser::from_url("https://thefullsnack.com/rss.xml")  {
    println!("{:?}", feed);
  }
}
```
