mod fslib;

use std::path::Path;

use fslib::{SyncFsClient, TokioFsClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // check if `tokio` argument
    let tokio = std::env::args().any(|arg| arg == "--tokio");
    if (tokio && std::env::args().len() < 3) || std::env::args().len() < 2 {
        eprintln!("Usage: example [--tokio] <file_path>");
        std::process::exit(1);
    }

    let file_path = std::env::args().next_back().unwrap_or_else(|| {
        eprintln!("Usage: example [--tokio] <file_path>");
        std::process::exit(1);
    });

    let path = Path::new(&file_path);

    if tokio {
        // If the `--tokio` argument is passed, run the async main function
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()?
            .block_on(tokio_main(path))
    } else {
        // Otherwise, run the sync main function
        sync_main(path)
    }
}

fn sync_main(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running in sync mode");

    let client = SyncFsClient::new(path);
    client.create()?;
    println!("File created at: {}", path.display());

    // write
    let data = b"Hello, world!";
    client.write(data)?;
    println!("Data written to file: {}", String::from_utf8_lossy(data));

    // read
    let read_data = client.read()?;
    println!(
        "Data read from file: {}",
        String::from_utf8_lossy(&read_data)
    );
    assert_eq!(
        read_data, data,
        "Data read from file does not match written data"
    );

    Ok(())
}

async fn tokio_main(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running in async mode");

    let client = TokioFsClient::new(path);
    client.create().await?;
    println!("File created at: {}", path.display());

    // write
    let data = b"Hello, world!";
    client.write(data).await?;
    println!("Data written to file: {}", String::from_utf8_lossy(data));

    // read
    let read_data = client.read().await?;
    println!(
        "Data read from file: {}",
        String::from_utf8_lossy(&read_data)
    );
    assert_eq!(
        read_data, data,
        "Data read from file does not match written data"
    );

    Ok(())
}
