use std::fmt::Display;

pub fn call1() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {}, ({})", self.headline, self.author, self.location)
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
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("러스트 언어 공부를 시작했습니다."),
        reply: false,
        retweet: false,
    };
    println!("새 트윗 1개: {}", tweet.summarize());
}

pub fn call2() {
    pub trait Summary {
        fn summarize(&self) -> String {
            // 기본 구현
            String::from("(계속 읽기)")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for NewsArticle {}

    let article = NewsArticle {
        headline: String::from("대한민국, 러시아 월드컵 예선에서 독일을 이겼다."),
        location: String::from("카잔 아레나, 러시아"),
        author: String::from("위키백과"),
        content: String::from(
            "2018년 6월 27일 러시아 카잔의 카잔 아레나에서 열린 2018년 월드컵...",
        ),
    };
    println!("새로운 기사: {}", article.summarize());
}

pub fn call3() {
    pub trait Summary {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            format!("{}님의 기사 더 읽기", self.summarize_author())
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("hourse_ebook"),
        content: String::from("러스트 언어 공부를 시작했습니다."),
        reply: false,
        retweet: false,
    };
    println!("새 트윗 1개: {}", tweet.summarize());
}

pub fn call4() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    pub fn notify1(item: impl Summary) {
        // Trait Bounds
        println!("속보! {}", item.summarize());
    }
    pub fn notify2<T: Summary>(item: T) {
        println!("속보! {}", item.summarize());
    }
    pub fn notify3(item1: impl Summary, item2: impl Summary) {
        println!("속보! {}", item1.summarize());
        println!("속보! {}", item2.summarize());
    }
    pub fn notify4<T: Summary>(item1: T, item2: T) {
        // Trait Bounds
        println!("속보! {}", item1.summarize());
        println!("속보! {}", item2.summarize());
    }
}

pub fn call5() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    pub trait Display {
        fn show(&self) -> String;
    }
    pub fn notify1(item: impl Summary + Display) {
        // need to implement
    }
    pub fn notify2<T: Summary + Display>(item: T) {
        // need to implement
    }
    pub fn some_function<T, U>(t: T, u: U) -> i32
    where
        T: Display + Clone,
        U: Summary + Clone,
    {
        // need to implement
        1
    }
}

pub fn call6() {
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(계속 읽기)")
        }
    }
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for NewsArticle {}

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    impl Summary for Tweet {}

    pub fn returns_summarizable1() -> impl Summary {
        Tweet {
            username: String::from("hourse_ebooks"),
            content: String::from("러스트 공부를 시작했습니다."),
            reply: false,
            retweet: false,
        }
    }

    // pub fn returns_summarizable2(switch: bool) -> impl Summary {
    //   if switch {
    //     NewsArticle {
    //       headline: String::from("대한민국, 러시아 월드컵 예선에서 독일을 이겼다."),
    //       location: String::from("카잔 아레나, 러시아"),
    //       author: String::from("위키백과"),
    //       content: String::from("2018년 6월 27일 러시아 카잔의 카잔 아레나에서 열린 2018년 월드컵..."),
    //     }
    //   } else {
    //     Tweet {
    //       username: String::from("hourse_ebook"),
    //       content: String::from("러스트 언어 공부를 시작했습니다."),
    //       reply: false,
    //       retweet: false,
    //     }
    //   }
    // }
}

pub fn call7() {
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("가장 큰 멤버는  x: {}", self.x);
            } else {
                println!("가장 큰 멤버는  y: {}", self.y);
            }
        }
    }
}
