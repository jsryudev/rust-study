pub trait Summarizable {
  fn author_summary(&self) -> String;
  fn summary(&self) -> String {
      format!("(Read more from {}...)", self.author_summary())
  }
}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

pub fn notify<T: Summarizable>(item: T) {
  println!("Breaking news! {}", item.summary());
}

//impl Summarizable for NewsArticle {
//  fn summary(&self) -> String {
//    format!("{}, by {} ({})", self.headline, self.author, self.location)
//  }
//}

//impl Summarizable for NewsArticle {}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

//impl Summarizable for WeatherForecast {
//    fn summary(&self) -> String {
//        format!("The high will be {}, and the low will be {}. The chance of
//        precipitation is {}%.", self.high_temp, self.low_temp,
//        self.chance_of_precipitation)
//    }
//}