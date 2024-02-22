use anyhow::{anyhow, Result};
use std::{collections::HashSet, fs::write, path::PathBuf, env::var};
use mkl_rs_build::build;

const OUT_DIR_ENV: &str = "OUT_DIR";
const MINIMUM_VERSION: &str = "2024";

macro_rules! mkl_config {
    ($cfg:literal) => {
        #[cfg(feature = $cfg)]
        const CONFIG: &str = concat!("mkl-", $cfg);
    };
}

mkl_config!("dynamic-ilp64-gomp");
mkl_config!("dynamic-ilp64-iomp");
mkl_config!("dynamic-ilp64-seq");
mkl_config!("dynamic-lp64-gomp");
mkl_config!("dynamic-lp64-iomp");
mkl_config!("dynamic-lp64-seq");
mkl_config!("static-ilp64-gomp");
mkl_config!("static-ilp64-iomp");
mkl_config!("static-ilp64-seq");
mkl_config!("static-lp64-gomp");
mkl_config!("static-lp64-iomp");
mkl_config!("static-lp64-seq");
mkl_config!("sdl");

#[cfg(not(any(
    feature = "dynamic-ilp64-gomp",
    feature = "dynamic-ilp64-iomp",
    feature = "dynamic-ilp64-seq",
    feature = "dynamic-lp64-gomp",
    feature = "dynamic-lp64-iomp",
    feature = "dynamic-lp64-seq",
    feature = "static-ilp64-gomp",
    feature = "static-ilp64-iomp",
    feature = "static-ilp64-seq",
    feature = "static-lp64-gomp",
    feature = "static-lp64-iomp",
    feature = "static-lp64-seq",
    feature = "sdl",
)))]
compile_error!("At least one of the following features must be enabled: dynamic-ilp64-gomp, dynamic-ilp64-iomp, dynamic-ilp64-seq, dynamic-ilp64-tbb, dynamic-lp64-gomp, dynamic-lp64-iomp, dynamic-lp64-seq, dynamic-lp64-tbb, static-ilp64-gomp, static-ilp64-iomp, static-ilp64-seq, static-ilp64-tbb, static-lp64-gomp, static-lp64-iomp, static-lp64-seq, static-lp64-tbb");

#[cfg(all(
    feature = "sdl",
    any(
        feature = "dynamic-ilp64-gomp",
        feature = "dynamic-ilp64-iomp",
        feature = "dynamic-ilp64-seq",
        feature = "dynamic-lp64-gomp",
        feature = "dynamic-lp64-iomp",
        feature = "dynamic-lp64-seq",
        feature = "static-ilp64-gomp",
        feature = "static-ilp64-iomp",
        feature = "static-ilp64-seq",
        feature = "static-lp64-gomp",
        feature = "static-lp64-iomp",
        feature = "static-lp64-seq",
    )
))]
compile_error!("Set default-features = false in Cargo.toml to use dynamic-sdl");

fn main() -> Result<()> {
    if cfg!(feature = "no-link") {
        Ok(())
    } else {
        build()
    }
}
