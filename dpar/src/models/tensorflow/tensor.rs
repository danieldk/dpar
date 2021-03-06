use std::ops::{Deref, DerefMut};

use enum_map::EnumMap;

use crate::features::Layer;
use tensorflow::{Tensor, TensorType};

/// Ad-hoc trait for shrinking batches.
pub trait ShrinkBatch {
    fn shrink_batch(&self, n_instances: u64) -> Self;
}

impl<T> ShrinkBatch for Tensor<T>
where
    T: Copy + TensorType,
{
    fn shrink_batch(&self, n_instances: u64) -> Self {
        assert!(
            n_instances <= self.dims()[0],
            "Trying to shrink batch of size {} to {}",
            self.dims()[0],
            n_instances
        );

        let mut new_shape = self.dims().to_owned();
        new_shape[0] = n_instances;
        let mut copy = Tensor::new(&new_shape);

        copy.copy_from_slice(&self[..new_shape.iter().cloned().product::<u64>() as usize]);

        copy
    }
}

impl<T> ShrinkBatch for TensorWrap<T>
where
    T: Copy + TensorType,
{
    fn shrink_batch(&self, n_instances: u64) -> Self {
        TensorWrap(self.0.shrink_batch(n_instances))
    }
}

impl<T> ShrinkBatch for LayerTensors<T>
where
    T: Copy + TensorType,
{
    fn shrink_batch(&self, n_instances: u64) -> Self {
        let mut copy = LayerTensors::new();

        // Note: EnumMap does not support FromIterator.
        for (layer, tensor) in self.iter() {
            copy[layer] = tensor.shrink_batch(n_instances);
        }

        copy
    }
}

/// Ad-hoc trait for converting extracting slices from tensors.
pub trait InstanceSlices<T> {
    /// Extract for each layer the slice corresponding to the `idx`-th
    /// instance from the batch.
    fn to_instance_slices(&mut self, idx: usize) -> EnumMap<Layer, &mut [T]>;
}

impl<T> InstanceSlices<T> for LayerTensors<T>
where
    T: TensorType,
{
    fn to_instance_slices(&mut self, idx: usize) -> EnumMap<Layer, &mut [T]> {
        let mut slices = EnumMap::new();

        for (layer, tensor) in self.iter_mut() {
            let layer_size = tensor.dims()[1] as usize;
            let offset = idx * layer_size;
            slices[layer] = &mut tensor[offset..offset + layer_size];
        }

        slices
    }
}

pub type LayerTensors<T> = EnumMap<Layer, TensorWrap<T>>;

/// Simple wrapper for `Tensor` that implements the `Default`
/// trait.
pub struct TensorWrap<T>(pub Tensor<T>)
where
    T: TensorType;

impl<T> Default for TensorWrap<T>
where
    T: TensorType,
{
    fn default() -> Self {
        TensorWrap(Tensor::new(&[]))
    }
}

impl<T> From<Tensor<T>> for TensorWrap<T>
where
    T: TensorType,
{
    fn from(tensor: Tensor<T>) -> Self {
        TensorWrap(tensor)
    }
}

impl<T> Deref for TensorWrap<T>
where
    T: TensorType,
{
    type Target = Tensor<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for TensorWrap<T>
where
    T: TensorType,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use tensorflow::Tensor;

    use super::ShrinkBatch;

    #[test]
    fn copy_batches() {
        let original = Tensor::new(&[4, 2])
            .with_values(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])
            .expect("Cannot initialize tensor.");
        let copy = original.shrink_batch(2);

        assert_eq!(&*copy, &[1.0, 2.0, 3.0, 4.0]);
    }
}
