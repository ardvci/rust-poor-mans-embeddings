# Poor Man's Embeddings: Semantic Search in Rust

This is a minimal Rust application that shows how to use sentence embeddings and cosine similarity to find the most relevant document from a collection. It uses the rust-bert crate with the lightweight AllMiniLM-L12-v2 transformer model, which runs quickly on CPU and has a small memory footprint.

---

## Features

- Embed text documents with AllMiniLM-L12-v2 (about 33M parameters, designed for speed and low resource use).
- Encode queries into the same vector space as the documents.
- Compare query and documents using cosine similarity.
- Return the most semantically relevant match.

---

## Project Structure
```
poor-mans-embeddings/
├─ Cargo.toml 
├─ assets/ 
│  └─ articles/
│     ├─ doc1.txt
│     ├─ doc2.txt
│     └─ doc3.txt
└─ src/
├─ main.rs
├─ reader/
│  ├─ mod.rs
│  └─ file_reader.rs
└─ query/
├─ mod.rs
└─ query_maker.rs
```
---

## Setup

Prerequisites:
- Rust (latest stable). Install via rustup.
- Internet connection (the model will be downloaded automatically).

Clone and build:
git clone https://github.com/<your-username>/poor-mans-embeddings.git
cd poor-mans-embeddings
cargo build --release

---

## Usage
1. Place your text files under `src/static/articles/`.
```
   Example:
   src/static/articles/
   ├── doc1.txt
   ├── doc2.txt
   └── doc3.txt
```
2. Run the binary:
   cargo run --release

3. Example output:
   full_path = /path/to/project/src/static/articles
   Cosine similarity: 0.8123
   Closest Article: "contents..."

---

## Example Query

In `main.rs` you will see:
let query = "<INSERT YOUR QUERY HERE>";

Change this string to test different queries against your article set.

---

## How It Works

1. Encode documents into embeddings (vectors in high-dimensional space).
2. Encode the query into the same space.
3. Compute cosine similarity between the query and each document vector.
4. Return the index of the document with the highest score.

Cosine similarity is simply the dot product divided by the product of vector lengths. It measures orientation rather than magnitude, which makes it well-suited for semantic tasks.

---

## Future Improvements

- Return top-k results instead of just the best match.
- Normalize embeddings up front to simplify similarity into a dot product.
- Parallelize scoring with rayon for larger collections.
- Integrate with a vector database such as Qdrant or Milvus for scalability.

---

Not bad for a poor man’s solution.