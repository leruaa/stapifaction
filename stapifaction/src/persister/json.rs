use std::{
    fs::{self, File},
    path::Path,
};

use erased_serde::Serialize;
use eyre::{Context, Result};

use crate::Persistable;

use super::Persister;

pub struct JsonPersister;

impl Persister for JsonPersister {
    fn write<'a>(&self, path: &Path, serializable: Box<dyn Serialize + 'a>) -> Result<()> {
        fs::create_dir_all(&path)?;

        let file_path = path.join("index.json");

        let file = File::create(&file_path)
            .wrap_err_with(|| format!("Failed to create file '{}'", file_path.display()))?;

        serde_json::to_writer(file, &serializable)
            .wrap_err_with(|| format!("Failed serialize element '{:?}'", file_path))?;

        Ok(())
    }
}

pub trait ToJson: Persistable + Sized {
    fn to_json<P: AsRef<Path>>(&self, base_path: P) -> Result<()> {
        let persister = JsonPersister;

        persister.persist(base_path.as_ref(), self)?;

        Ok(())
    }
}

impl<T: Persistable> ToJson for T {}

pub trait ToJsonIterable: IntoIterator + Sized
where
    <Self as IntoIterator>::Item: ToJson,
{
    fn to_json<P: AsRef<Path>>(self, base_path: P) -> Result<()> {
        self.into_iter()
            .try_for_each(|p| p.to_json(base_path.as_ref()))?;

        Ok(())
    }
}

impl<I: ToJson> ToJsonIterable for Vec<I> {}