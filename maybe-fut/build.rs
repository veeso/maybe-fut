use cfg_aliases::cfg_aliases;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup cfg aliases
    cfg_aliases! {
        // features
        tokio: { feature = "tokio" },
        tokio_fs: { feature = "tokio-fs" },
        tokio_net: { feature = "tokio-net" },
        tokio_sync: { feature = "tokio-sync" },
        tokio_time: { feature = "tokio-time" }
    }

    Ok(())
}
