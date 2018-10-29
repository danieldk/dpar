use std::ops::{Deref, DerefMut};

use enum_map::EnumMap;

use features::Layer;
use tensorflow::{Tensor, TensorType};

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
