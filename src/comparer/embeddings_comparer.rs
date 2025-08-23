use rust_bert::pipelines::sentence_embeddings::{Embedding};
use rust_bert::RustBertError;

pub struct EmbeddingsComparer;

impl EmbeddingsComparer {
    pub fn find_similar_embedding(
        base_embeddings_res: Result<Vec<Embedding>, RustBertError>,
        query_embeddings_res: Result<Vec<Embedding>, RustBertError>,
    ) -> Result<i32, RustBertError> {
        let base_embeddings = base_embeddings_res?;
        let query_embeddings = query_embeddings_res?;
        let query = query_embeddings
            .get(0)
            .ok_or_else(|| {
                RustBertError::InvalidConfigurationError(
                    "No comparer embeddings provided".into(),
                )
            })?;

        let query_slice: &[f32] = query.as_slice();
        let mut max_score = 0.0_f32;
        let mut best_idx = 0;

        for (i, embedding) in base_embeddings.iter().enumerate() {
            let emb_slice: &[f32] = embedding.as_slice();
            let sim = calculate_cosine_similarity(emb_slice, query_slice);
            if sim > max_score {
                max_score = sim;
                best_idx = i as i32;
            }
        }
        println!("Cosine Similarity: {}", max_score);
        Ok(best_idx)
    }
}

pub fn calculate_cosine_similarity(
    v1: &[f32],
    v2: &[f32]
) -> f32 {
    assert_eq!(v1.len(), v2.len(), "vectors must have the same length");
    let sum_of_multiplies: f32 = v1.iter().zip(v2.iter()).map(
        |(x,y)| (x * y)
    ).sum();

    let divider_x: f32 = v1.iter().map(
        |x| x.powi(2)
    ).sum::<f32>().sqrt();

    let divider_y: f32 = v2.iter().map(
        |y| y.powi(2)
    ).sum::<f32>().sqrt();

    let result = sum_of_multiplies / (divider_x * divider_y);
    result
}
