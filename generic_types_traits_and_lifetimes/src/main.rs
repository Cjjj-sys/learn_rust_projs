use std::fmt::Display;

fn main() {
    let news = News::new("cool");
    news.show();
    let article = Article::new(114514);
    article.show();

    append_article_to_news_and_show(&article, &news);
    show_show(&news);
    show_show(&article);
}

trait Show {
    fn show(&self);
}

struct News<T> {
    content: T
}

struct Article<T> {
    content: T
}

impl<T> News<T> {
    fn new(content: T) -> News<T> {
        Self {
            content
        }
    }
}

impl<T: Display> Show for News<T> {
    fn show(&self) {
        println!("{}", self.content)
    }
}

impl<T> Article<T> {
    fn new(content: T) -> Article<T> {
        Self {
            content
        }
    }
}

impl<T: Display> Show for Article<T> {
    fn show(&self) {
        println!("Article: {}", self.content)
    }
}

fn append_article_to_news_and_show<T, U>(article: &T, news: &U) where
    T: Show,
    U: Show {
    news.show();
    article.show();
}

fn show_show(s: &impl Show) {
    s.show();
}