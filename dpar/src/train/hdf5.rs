use std::collections::HashMap;
use std::path::Path;

use hdf5;
use hdf5::IntoData;

use Result;
use features::{InputVector, InputVectorizer};
use system::{ParserState, TransitionSystem};
use train::InstanceCollector;

pub struct HDF5Collector<T>
where
    T: TransitionSystem,
{
    writer: HDF5Writer,
    transition_system: T,
    vectorizer: InputVectorizer,
}

impl<T> HDF5Collector<T>
where
    T: TransitionSystem,
{
    pub fn new<P>(
        transition_system: T,
        hdf5_path: P,
        vectorizer: InputVectorizer,
        batch_size: usize,
    ) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(HDF5Collector {
            writer: HDF5Writer::new(hdf5::File::new(hdf5_path)?, batch_size),
            transition_system: transition_system,
            vectorizer: vectorizer,
        })
    }

    pub fn transition_system(&self) -> &T {
        &self.transition_system
    }
}

impl<T> InstanceCollector<T> for HDF5Collector<T>
where
    T: TransitionSystem,
{
    fn collect(&mut self, t: &T::T, state: &ParserState) -> Result<()> {
        let label = self.transition_system.transitions_mut().add(t.clone());
        let v = self.vectorizer.realize(state);
        self.writer.write(label, v)
    }
}

pub struct HDF5Writer {
    file: hdf5::File,
    batch: usize,
    batch_size: usize,
    batch_idx: usize,
    labels: Vec<i32>,
    first: bool,
    data: HashMap<String, Vec<i32>>,
}

impl HDF5Writer {
    pub fn new(hdf5_file: hdf5::File, batch_size: usize) -> Self {
        HDF5Writer {
            file: hdf5_file,
            batch: 0,
            batch_size: batch_size,
            batch_idx: 0,
            labels: vec![0; batch_size],
            first: true,
            data: HashMap::new(),
        }
    }

    pub fn write(&mut self, label: usize, input: InputVector) -> Result<()> {
        if self.first {
            for (layer, data) in &input.layers {
                self.data
                    .insert(layer.to_string(), vec![0; self.batch_size * data.len()]);
            }

            self.first = false
        }

        for (layer, data) in &input.layers {
            let start_idx = self.batch_idx * data.len();
            self.data.get_mut(&layer.to_string()).unwrap()[start_idx..start_idx + data.len()]
                .copy_from_slice(data);
        }

        self.labels[self.batch_idx] = label as i32;

        self.batch_idx += 1;

        if self.batch_idx >= self.batch_size {
            self.write_batch()?;
            self.clear_batch();
        }

        Ok(())
    }

    fn clear_batch(&mut self) {
        self.batch_idx = 0;
        self.batch += 1;

        for data in self.data.values_mut() {
            for v in data.iter_mut() {
                *v = 0;
            }
        }

        for v in self.labels.iter_mut() {
            *v = 0;
        }
    }

    fn write_batch(&mut self) -> Result<()> {
        for (layer, data) in self.data.iter_mut() {
            let input_size = data.len() / self.batch_size;
            let mut writer = hdf5::Writer::new(
                &self.file,
                &format!("batch{}-{}", self.batch, layer),
                &[self.batch_size, input_size],
            );
            writer.write(data.into_data()?, &[0, 0], &[self.batch_size, input_size])?;
        }

        let mut writer = hdf5::Writer::new(
            &self.file,
            &format!("batch{}-labels", self.batch),
            &[self.batch_size],
        );
        writer.write(self.labels.into_data()?, &[0], &[self.batch_size])?;

        Ok(())
    }
}

impl Drop for HDF5Writer {
    fn drop(&mut self) {
        self.write_batch().unwrap();
        self.file.write("batches", self.batch + 1).unwrap();
    }
}
