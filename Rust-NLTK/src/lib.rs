use rsnltk::sentiment;
pub fn sentiment_out(input: &str) {
    
    let lang = "en";
    let sentiments = sentiment(&input, lang);
    println!("{:?}", sen);
}
