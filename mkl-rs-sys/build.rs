use anyhow::{anyhow, Result};
use pkg_config::Config;
use bindgen::{MacroTypeVariation, NonCopyUnionStyle, EnumVariation, AliasVariation, FieldVisibilityKind, Builder};
use std::{collections::HashSet, path::PathBuf, env::var};

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
    let library = Config::new()
        .atleast_version(MINIMUM_VERSION)
        .probe(CONFIG)?;

    let lib_paths = library.link_files.iter()
        .filter_map(|p| p.parent().map(|p| p.to_path_buf()))
        .chain(library.link_paths.iter().map(|p| p.to_path_buf()))
        .collect::<HashSet<_>>();

    let ld_library_paths = lib_paths.iter()
        .filter_map(|p| p.to_str())
        .map(|p| p.to_string())
        .collect::<Vec<_>>();

    let include_path = library.include_paths
        .first()
        .ok_or_else(|| anyhow!("No include path found"))?;

    let mkl_include_path = include_path
        .join("mkl.h");

    Builder::default()
        .clang_arg(format!("-I{}", &include_path.to_str().ok_or_else(|| anyhow!("Invalid include path"))?))
        .clang_arg("-fretain-comments-from-system-headers")
        .clang_arg("-fparse-all-comments")
        // We don't care at all what warnings simics has if they aren't errors :)
        .clang_arg("-Wno-everything")
        .default_visibility(FieldVisibilityKind::Public)
        .default_alias_style(AliasVariation::TypeAlias)
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .default_macro_constant_type(MacroTypeVariation::Unsigned)
        .default_non_copy_union_style(NonCopyUnionStyle::BindgenWrapper)
        .derive_default(true)
        .derive_hash(true)
        .derive_partialord(true)
        .derive_ord(true)
        .derive_eq(true)
        .derive_partialeq(true)
        .generate_comments(true)
        // Uses 128-bit unstable
        .blocklist_function("strtold")
        .blocklist_function("qecvt")
        .blocklist_function("qfcvt")
        .blocklist_function("qgcvt")
        .blocklist_function("qecvt_r")
        .blocklist_function("qfcvt_r")
        .header(mkl_include_path.to_str().ok_or_else(|| anyhow!("Invalid include path"))?)
        .generate()
        .map_err(|e| anyhow!("Failed to generate bindings: {}", e))?
        .write_to_file(PathBuf::from(var(OUT_DIR_ENV)?).join("bindings.rs"))?;

    // Link to gomp if a gomp feature is enabled
    if cfg!(feature = "dynamic-ilp64-gomp") || cfg!(feature = "dynamic-lp64-gomp") || cfg!(feature = "static-ilp64-gomp") || cfg!(feature = "static-lp64-gomp") {
        println!("cargo:rustc-link-lib=gomp");
    }

    // Link to iomp if a iomp feature is enabled
    if cfg!(feature = "static-ilp64-iomp") || cfg!(feature = "static-lp64-iomp") {
        println!("cargo:rustc-link-lib=iomp5");
    }

    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", ld_library_paths.join(":"));

    Ok(())
}
