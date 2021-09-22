pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}({})", self.headline, self.author, self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news!{}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably"),
        reply: false,
        retweet: false,
    };
    notify(tweet);
    let article = NewArticle {
        headline: String::from("Penguins win"),
        location: String::from("USA"),
        author: String::from("Iceburgh"),
        content: String::from("nhk"),
    };
    notify(article);
}
