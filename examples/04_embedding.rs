use naturallanguage::Embedding;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(emb) = Embedding::word_for_language("en")? else {
        eprintln!("no English word embedding available");
        return Ok(());
    };
    println!(
        "dimension = {}, vocab = {}",
        emb.dimension(),
        emb.vocabulary_size()
    );

    if let Some(d) = emb.distance("king", "queen")? {
        println!("dist(king, queen) = {d:.4}");
    }
    if let Some(d) = emb.distance("king", "pizza")? {
        println!("dist(king, pizza) = {d:.4}");
    }

    println!("nearest to 'computer':");
    for n in emb.neighbors("computer", 5)? {
        println!("  {} ({:.4})", n.word, n.distance);
    }
    Ok(())
}
