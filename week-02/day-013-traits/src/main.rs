mod aggregator;
mod conditional_traits;

pub fn notify<T>(item: &T)
    where T: aggregator::Summary {
    println!("Fake news! {}", item.summarize());
}

fn tweet_factory(user: &str, content: &str) -> impl aggregator::Summary {
    aggregator::Tweet {
        username: String::from(user),
        content: String::from(content),
        reply: false,
        retweet: false,
    }
}

fn conditional_traits_method_implementation() {
    let p = conditional_traits::Pair::new(69, 420);
    p.cmp_display();
}

fn main() {
    let tweet = tweet_factory("vgebrev", "280 characters is not enough for meaningful discourse");
    notify(&tweet);

    let article = aggregator::NewsArticle {
        author: String::from("Seymore Butts"),
        headline: String::from("Americans launch into space"),
        content: String::from("Unfortunately, not all of them"),
        location: String::from("Florida"),
    };

    notify(&article);

    conditional_traits_method_implementation();
}
