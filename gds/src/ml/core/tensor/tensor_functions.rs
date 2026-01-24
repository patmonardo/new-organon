//! Tensor functions utilities.
//!
//! Translated from Java GDS ml-core TensorFunctions.java.

use crate::ml::core::Tensor;

/// Average multiple lists of tensors.
///
/// The operation is done in-place on the first entry of the input and returned.
pub fn average_tensors(batched_tensors: Vec<Vec<Box<dyn Tensor>>>) -> Vec<Box<dyn Tensor>> {
    let number_of_batches = batched_tensors.len();
    average_tensors_with_batches(batched_tensors, number_of_batches)
}

/// Average multiple lists of tensors with an explicit batch count.
///
/// The operation is done in-place on the first entry of the input and returned.
pub fn average_tensors_with_batches(
    mut batched_tensors: Vec<Vec<Box<dyn Tensor>>>,
    number_of_batches: usize,
) -> Vec<Box<dyn Tensor>> {
    if batched_tensors.is_empty() {
        return Vec::new();
    }
    if number_of_batches == 0 {
        panic!("number_of_batches must be greater than 0");
    }

    let mut mean_tensors = batched_tensors.remove(0);

    for i in 0..mean_tensors.len() {
        for batch in batched_tensors.iter() {
            let weighted_batch_tensor = batch[i].as_ref();
            mean_tensors[i].add_inplace(weighted_batch_tensor);
        }
        let scaled = mean_tensors[i].scalar_multiply(1.0 / number_of_batches as f64);
        mean_tensors[i] = scaled;
    }

    mean_tensors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::core::Scalar;

    #[test]
    fn test_average_tensors() {
        let batch1 = vec![Box::new(Scalar::new(1.0)) as Box<dyn Tensor>];
        let batch2 = vec![Box::new(Scalar::new(3.0)) as Box<dyn Tensor>];

        let averaged = average_tensors(vec![batch1, batch2]);
        let scalar = averaged[0]
            .as_any()
            .downcast_ref::<Scalar>()
            .expect("Expected Scalar");
        assert_eq!(scalar.value(), 2.0);
    }
}
