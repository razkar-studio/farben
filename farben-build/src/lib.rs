//! Build-time support crate for loading and parsing `.frb.toml` config files.
//!
//! This crate is used by `build.rs` scripts to compile farben style definitions
//! into the binary at build time.
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]

pub mod core;
mod parser;

use std::{collections::HashMap, fs, path::Path};

/// Runs the build script with the default config filename `farben.frb.toml`.
pub fn run() {
    run_with(&["farben.frb.toml"]);
}

/// Runs the build script with custom config file paths.
pub fn run_with(paths: &[&str]) {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("farben_styles.rs");

    for path in paths {
        println!("cargo:rerun-if-changed={path}");
    }

    let mut all_styles: HashMap<String, String> = HashMap::new();
    let mut all_prefixes: HashMap<String, String> = HashMap::new();

    for path in paths {
        let config = parser::parse(
            &fs::read_to_string(path)
                .unwrap_or_else(|_| panic!("farben-build: could not find '{path}'")),
        )
        .unwrap_or_else(|e| panic!("farben-build: error parsing '{path}': {e}"));

        for (key, value) in config.styles {
            if all_styles.contains_key(&key) {
                panic!("farben-build: duplicate style '{key}' found in '{path}'");
            }
            all_styles.insert(key, value);
        }

        for (key, value) in config.prefixes {
            if all_prefixes.contains_key(&key) {
                panic!("farben-build: duplicate prefix '{key}' found in '{path}'");
            }
            all_prefixes.insert(key, value);
        }
    }

    let mut code = String::new();

    code.push_str("pub fn init_styles() {\n");

    for (key, value) in &all_styles {
        code.push_str(&format!(
            "    farben::insert_style({key:?}, farben::Style::parse({markup:?}).unwrap_or_else(|e| panic!(\"{{e}}\")));\n",
            markup = format!("[{value}]")
        ));
    }

    for (key, value) in &all_prefixes {
        code.push_str(&format!(
            "    farben::set_prefix({key:?}, {value:?}).unwrap_or_else(|e| panic!(\"{{e}}\"));\n"
        ));
    }

    code.push_str("}\n");

    std::fs::write(&dest, code)
        .unwrap_or_else(|e| panic!("farben-build: could not write generated file: {e}"));

    /*
     * LSV: line separated values
     *  A custom data format intentionally made dead simple
     *  so my parser logic would also be dead simple. No spaces between =
     * Why LSV?
     *      I don't want to write any other parser.
     * Why write a parser anyway?
     *      Farben needs to not have any dependencies.
     */
    let registry_dest = Path::new(&out_dir).join("farben_registry.lsv");
    let mut registry_content = all_styles
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("\n");
    registry_content.push_str("\n---\n");
    registry_content.push_str(
        &all_prefixes
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    std::fs::write(&registry_dest, registry_content)
        .unwrap_or_else(|e| panic!("farben-build: could not write registry file: {e}"));
}
