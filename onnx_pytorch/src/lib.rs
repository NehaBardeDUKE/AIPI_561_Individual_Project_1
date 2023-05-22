use anyhow::Result;
use rust_bert::pipelines::sentiment::SentimentModel;
pub fn analyze_sent() -> anyhow::Result<()> {
    let sentiment_model = SentimentModel::new(Default::default())?;
    let input1= ["Probably my all-time favorite movie, a story of selflessness, sacrifice and dedication to a noble cause, but it's not preachy or boring.",
    "This film tried to be too many things all at once: stinging political satire, Hollywood blockbuster, sappy romantic comedy, family values promo..."];
    let output = sentiment_model.predict(input1);
    println!("{:?}", output);
    Ok(())
}
