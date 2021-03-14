use traits::{Summarizable, Tweet, NewsArticle};
use std::cmp::PartialOrd;

fn main() {
    //let tweet = Tweet {
    //    username: String::from("horse_ebooks"),
    //    content: String::from("of course, as you probably already know, people"),
    //    reply: false,
    //    retweet: false,
    //};
    
    //println!("1 new tweet: {}", tweet.summary());

    //let article = NewsArticle {
    //    headline: String::from("Penguins win the Stanley Cup Championship!"),
    //    location: String::from("Pittsburgh, PA, USA"),
    //    author: String::from("Iceburgh"),
    //    content: String::from("The Pittsburgh Penguins once again are the best
    //    hockey team in the NHL."),
    //};

    //println!("New article available! {}", article.summary());
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}