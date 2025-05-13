use std::path::{Path, PathBuf};

use maybe_fut::fs::File;

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
}
