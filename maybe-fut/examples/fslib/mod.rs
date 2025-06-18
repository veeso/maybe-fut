use std::path::{Path, PathBuf};

use maybe_fut::fs::File;
use maybe_fut::io::{Read, Write};

struct FsClient {
    path: PathBuf,
}

#[maybe_fut::maybe_fut(
    sync = SyncFsClient,
    tokio = TokioFsClient,
    tokio_feature = "tokio"
)]
impl FsClient {
    /// Creates a new `FsClient` instance.
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Creates a new file at the specified path.
    pub async fn create(&self) -> std::io::Result<()> {
        // Create a new file at the specified path.
        let file = File::create(&self.path).await?;
        file.sync_all().await?;

        Ok(())
    }

    /// Write data to the file at the specified path.
    pub async fn write(&self, data: &[u8]) -> std::io::Result<()> {
        // Open the file in write mode and write the data to it.
        let mut file = File::create(&self.path).await?;
        file.write_all(data).await?;
        file.sync_all().await?;

        Ok(())
    }

    pub async fn read(&self) -> std::io::Result<Vec<u8>> {
        // Open the file in read mode and read the data from it.
        let mut file = File::open(&self.path).await?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        Ok(buffer)
    }
}
