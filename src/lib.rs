use rust_bert::pipelines::sentiment::SentimentModel;
pub fn analyze_sent(runtime_value: &str) -> anyhow::Result<()> {
    let sentiment_model = SentimentModel::new(Default::default())?;
    let pre_populated_value = "Sentiment Analysis";
    //let runtime_value: &str = "World"; // Replace this with the logic to get the runtime value

    let my_array: [&str; 2] = [pre_populated_value, runtime_value];

    let output = sentiment_model.predict(my_array);
    println!("{:?}", output[1]);
    Ok(())
}
