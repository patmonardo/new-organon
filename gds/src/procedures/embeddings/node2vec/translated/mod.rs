// Internal adapter module that reuses the Cursor-translated Node2Vec internals
// without directly exposing the `embeddings.java` tree as public API.

#[path = "../../../embeddings.java/node2vec/compressed_random_walks.rs"]
pub mod compressed_random_walks;

#[path = "../../../embeddings.java/node2vec/negative_sample_producer.rs"]
pub mod negative_sample_producer;

#[path = "../../../embeddings.java/node2vec/positive_sample_producer.rs"]
pub mod positive_sample_producer;

#[path = "../../../embeddings.java/node2vec/random_walk_probabilities.rs"]
pub mod random_walk_probabilities;

#[path = "../../../embeddings.java/node2vec/sampling_walk_parameters.rs"]
pub mod sampling_walk_parameters;

#[path = "../../../embeddings.java/node2vec/train_parameters.rs"]
pub mod train_parameters;

#[path = "../../../embeddings.java/node2vec/training_task.rs"]
pub mod training_task;

#[path = "../../../embeddings.java/node2vec/node2vec_model.rs"]
pub mod node2vec_model;

#[path = "../../../embeddings.java/node2vec/node2vec_parameters.rs"]
pub mod node2vec_parameters;

#[path = "../../../embeddings.java/node2vec/node2vec_result.rs"]
pub mod node2vec_result;
