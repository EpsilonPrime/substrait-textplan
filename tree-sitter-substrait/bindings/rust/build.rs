// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

fn main() {
    let src_dir = Path::new("src");
    
    let mut c_config = cc::Build::new();
    c_config.include(&src_dir);
    
    let parser_path = src_dir.join("parser.c");
    c_config.file(&parser_path);
    
    if cfg!(target_os = "macos") {
        c_config.flag_if_supported("-Wno-unused-parameter");
        c_config.flag_if_supported("-Wno-unused-but-set-variable");
        c_config.flag_if_supported("-Wno-trigraphs");
    }
    
    c_config.compile("tree-sitter-substrait");
    
    // Regenerate the parser if grammar.js is modified
    println!("cargo:rerun-if-changed=grammar.js");
}