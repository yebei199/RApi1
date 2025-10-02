use my_lib_1::study_trait::{NewsArticle, Thh};

fn main() {
    my_lib_1::axum_first::hh();
    let a1 = my_lib_1::study_trait::NewsArticle {
        headline: "Penguins win the Stanley Cup".to_string(),
        location: "Pittsburgh, PA, USA".to_string(),
        author: "The Red Wings".to_string(),
        content: "The Pittsburgh Penguins once again are the best hockey team in the NHL.".to_string(),
    };
    a1.thh();
    NewsArticle::not()
}
