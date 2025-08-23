mod reader;
mod comparer;
use anyhow::Result;
use std::{fs, env};
use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};
use rust_poor_man_embeddings::reader::file_reader::FileReader;
use rust_poor_man_embeddings::comparer::embeddings_comparer::EmbeddingsComparer;




fn main() -> Result<()> {
    let binding = env::current_dir()?;
    let current_pwd = binding.to_str().expect("working directory path wasn’t valid UTF‑8");
    let query = "<INSERT YOUR QUERY HERE>";
    let full_path = current_pwd.to_owned() + "/src/static/articles";
    let paths = fs::read_dir(full_path)?;
    let model = SentenceEmbeddingsBuilder::remote(
        SentenceEmbeddingsModelType::AllMiniLmL12V2
    ).create_model()?;


    let mut sentences: Vec<String> = Vec::new();
    for path in paths {
        let path = path?.path();
        let f_reader = FileReader;
        let read_result = f_reader.read_to_string(
            path.to_str().unwrap()
        );
        sentences.push(read_result?.to_string());
    }

    let embedding_output = model.encode(&sentences);
    let query_embedding = model.encode(&[query]);
    let idx: i32 = EmbeddingsComparer::find_similar_embedding(
        embedding_output, query_embedding
    )?;

    println!("Closest Article: {:?}",sentences.get(idx as usize));



    Ok(())
}
