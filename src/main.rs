use clap::Parser;
use tracing_subscriber::{EnvFilter, filter::LevelFilter, fmt, prelude::*};

use oarfish::{prog_opts::Args, quantify};

// Genome-projection mapping churns many small allocations across worker threads;
// glibc malloc fragments/retains that into large peak RSS. mimalloc keeps the
// peak down and returns memory to the OS. (Disable with `--no-default-features`.)
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> anyhow::Result<()> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    let (filtered_layer, reload_handle) = tracing_subscriber::reload::Layer::new(env_filter);

    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stderr))
        .with(filtered_layer)
        .init();

    let args = Args::parse();
    quantify(args, &|filter| {
        reload_handle.modify(|current| *current = filter)?;
        Ok(())
    })
}
