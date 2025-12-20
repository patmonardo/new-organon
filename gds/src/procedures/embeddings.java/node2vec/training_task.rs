use super::negative_sample_producer::NegativeSampleProducer;
use super::positive_sample_producer::PositiveSampleProducer;
use super::node2vec_model::EPSILON;
use crate::collections::HugeObjectArray;
use crate::ml::core::functions::sigmoid::Sigmoid;

/// Training task that updates center/context embeddings using skip-gram with negative sampling.
///
/// Java: `TrainingTask implements Runnable`
pub struct TrainingTask<'a> {
    center_embeddings: &'a mut HugeObjectArray<Vec<f32>>,
    context_embeddings: &'a mut HugeObjectArray<Vec<f32>>,
    positive_sample_producer: PositiveSampleProducer<'a>,
    negative_sample_producer: NegativeSampleProducer,
    center_gradient_buffer: Vec<f32>,
    context_gradient_buffer: Vec<f32>,
    negative_sampling_rate: usize,
    learning_rate: f32,
    loss_sum: f64,
}

impl<'a> TrainingTask<'a> {
    pub fn new(
        center_embeddings: &'a mut HugeObjectArray<Vec<f32>>,
        context_embeddings: &'a mut HugeObjectArray<Vec<f32>>,
        positive_sample_producer: PositiveSampleProducer<'a>,
        negative_sample_producer: NegativeSampleProducer,
        learning_rate: f32,
        negative_sampling_rate: usize,
        embedding_dimensions: usize,
    ) -> Self {
        Self {
            center_embeddings,
            context_embeddings,
            positive_sample_producer,
            negative_sample_producer,
            center_gradient_buffer: vec![0.0; embedding_dimensions],
            context_gradient_buffer: vec![0.0; embedding_dimensions],
            negative_sampling_rate,
            learning_rate,
            loss_sum: 0.0,
        }
    }

    pub fn run(&mut self) {
        let mut buffer = [0i64; 2];
        while self.positive_sample_producer.next(&mut buffer) {
            self.train_positive_sample(buffer[0], buffer[1]);
            for _ in 0..self.negative_sampling_rate {
                let neg = self.negative_sample_producer.next();
                self.train_negative_sample(buffer[0], neg);
            }
        }
    }

    pub fn loss_sum(&self) -> f64 {
        self.loss_sum
    }

    fn train_positive_sample(&mut self, center: i64, context: i64) {
        let scaled_gradient = self.compute_positive_gradient(center as usize, context as usize);
        self.update_embeddings(center as usize, context as usize, scaled_gradient);
    }

    fn train_negative_sample(&mut self, center: i64, context: i64) {
        let scaled_gradient = self.compute_negative_gradient(center as usize, context as usize);
        self.update_embeddings(center as usize, context as usize, scaled_gradient);
    }

    fn compute_positive_gradient(&mut self, center: usize, context: usize) -> f32 {
        let affinity = inner_product(
            self.center_embeddings.get(center),
            self.context_embeddings.get(context),
        ) as f64;

        let positive_sigmoid = Sigmoid::sigmoid(affinity);
        let negative_sigmoid = 1.0 - positive_sigmoid;
        self.loss_sum -= (positive_sigmoid + EPSILON).ln();

        // Java: gradient = -negativeSigmoid; return -gradient * lr = negativeSigmoid * lr
        (negative_sigmoid as f32) * self.learning_rate
    }

    fn compute_negative_gradient(&mut self, center: usize, context: usize) -> f32 {
        let affinity = inner_product(
            self.center_embeddings.get(center),
            self.context_embeddings.get(context),
        ) as f64;

        let positive_sigmoid = Sigmoid::sigmoid(affinity);
        let negative_sigmoid = 1.0 - positive_sigmoid;
        self.loss_sum -= (negative_sigmoid + EPSILON).ln();

        // Java: gradient = positiveSigmoid; return -gradient * lr
        -(positive_sigmoid as f32) * self.learning_rate
    }

    fn update_embeddings(&mut self, center: usize, context: usize, scaled_gradient: f32) {
        // center_grad = scaled_gradient * context_embedding
        // context_grad = scaled_gradient * center_embedding
        let center_embedding_snapshot = self.center_embeddings.get(center).clone();
        let context_embedding_snapshot = self.context_embeddings.get(context).clone();

        scale_to(
            &context_embedding_snapshot,
            scaled_gradient,
            &mut self.center_gradient_buffer,
        );
        scale_to(
            &center_embedding_snapshot,
            scaled_gradient,
            &mut self.context_gradient_buffer,
        );

        add_in_place(
            self.center_embeddings.get_mut(center),
            &self.center_gradient_buffer,
        );
        add_in_place(
            self.context_embeddings.get_mut(context),
            &self.context_gradient_buffer,
        );
    }
}

fn inner_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

fn scale_to(input: &[f32], scalar: f32, out: &mut [f32]) {
    assert_eq!(input.len(), out.len());
    for i in 0..input.len() {
        out[i] = input[i] * scalar;
    }
}

fn add_in_place(lhs: &mut [f32], rhs: &[f32]) {
    let len = lhs.len().min(rhs.len());
    for i in 0..len {
        lhs[i] += rhs[i];
    }
}


