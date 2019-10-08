// Trait -> interface similar
trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more {}...)", self.summarize_auth())
    }
    fn summarize_auth(&self) -> String;
}

struct NewsArticle{
    headline: String,
    location: String,
    author: String,
    content: String
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summary for Tweet {
    fn summarize_auth(&self) -> String { 
        return format!("{}, by {} ({}) -> {}", self.username, self.content, self.reply, self.retweet);
    }
}

impl Summary for NewsArticle {
    fn summarize_auth(&self) -> String { 
        return format!("@{}", self.author);
    }
}

fn main() {
    let news = NewsArticle {
        headline: String::from("We won"),
        location: String::from("Australia"),
        author: String::from("Diwakar"),
        content: String::from("We won world cup"),
    };

    let tweet = Tweet {
        username: String::from("We won"),
        reply: true,
        retweet: false,
        content: String::from("We won world cup"),
    };
    println!("News Article\n{}", news.summarize());
}