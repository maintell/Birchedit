//! Birchpad's tree-sitter grammars in libraries of their own (ADR 0070).
//!
//! Built with every grammar, this is one library the executable loads; built with one of the
//! `lang-*` features, it is that language's pack, which the executable looks for first
//! (`crates/syntax/src/dynamic.rs`). A language's id names a grammar in both.

#![allow(
    unsafe_code,
    reason = "the one exported function is C ABI: a null-terminated id in, a function pointer \
              to the grammar out, which is how tree-sitter's grammars are handed over"
)]

use std::ffi::{CStr, c_char, c_void};

/// The function that returns the grammar of `id`, a Birchpad language id, as a function
/// pointer; null when this library has no such language.
///
/// The pointer is the C function the grammar crate exports (`tree_sitter_rust`); calling it
/// gives the `TSLanguage`. It stays valid while this library is loaded, which the caller keeps
/// it to be.
///
/// # Safety
///
/// `id` is null or a null-terminated C string that outlives the call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn birchpad_grammar(id: *const c_char) -> *mut c_void {
    if id.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: the caller promises a null-terminated string.
    let Ok(id) = unsafe { CStr::from_ptr(id) }.to_str() else {
        return std::ptr::null_mut();
    };
    let grammar: *const () = match id {
        #[cfg(feature = "tree-sitter-asm")]
        "asm" => tree_sitter_asm::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-bash")]
        "bash" => tree_sitter_bash::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-batch")]
        "batch" => tree_sitter_batch::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-c")]
        "c" => tree_sitter_c::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-c-sharp")]
        "cs" => tree_sitter_c_sharp::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-cpp")]
        "cpp" => tree_sitter_cpp::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-cmake")]
        "cmake" => tree_sitter_cmake::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-css")]
        "css" => tree_sitter_css::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-dart")]
        "dart" => tree_sitter_dart::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-diff")]
        "diff" => tree_sitter_diff::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-containerfile")]
        "dockerfile" => tree_sitter_containerfile::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-elixir")]
        "elixir" => tree_sitter_elixir::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-erlang")]
        "erlang" => tree_sitter_erlang::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-go")]
        "go" => tree_sitter_go::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-graphql")]
        "graphql" => tree_sitter_graphql::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-groovy")]
        "groovy" => tree_sitter_groovy::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-hcl")]
        "hcl" => tree_sitter_hcl::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-html")]
        "html" => tree_sitter_html::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-ini")]
        "ini" => tree_sitter_ini::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-java")]
        "java" => tree_sitter_java::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-javascript")]
        "javascript" => tree_sitter_javascript::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-json")]
        "json" => tree_sitter_json::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-kotlin-sg")]
        "kotlin" => tree_sitter_kotlin_sg::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-lua")]
        "lua" => tree_sitter_lua::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-make")]
        "makefile" => tree_sitter_make::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-md")]
        "markdown" => tree_sitter_md::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-nginx")]
        "nginx" => tree_sitter_nginx::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-nix")]
        "nix" => tree_sitter_nix::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-objc")]
        "objc" => tree_sitter_objc::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-pascal")]
        "pascal" => tree_sitter_pascal::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "birchpad-tree-sitter-plantuml")]
        "plantuml" => birchpad_tree_sitter_plantuml::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-powershell")]
        "powershell" => tree_sitter_powershell::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-properties")]
        "properties" => tree_sitter_properties::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-proto")]
        "proto" => tree_sitter_proto::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-python")]
        "python" => tree_sitter_python::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-r")]
        "r" => tree_sitter_r::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-ruby")]
        "ruby" => tree_sitter_ruby::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-rust")]
        "rust" => tree_sitter_rust::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-solidity")]
        "solidity" => tree_sitter_solidity::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-sequel")]
        "sql" => tree_sitter_sequel::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-svelte-ng")]
        "svelte" => tree_sitter_svelte_ng::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-swift")]
        "swift" => tree_sitter_swift::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-toml-ng")]
        "toml" => tree_sitter_toml_ng::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-typescript")]
        "typescript" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-typescript")]
        "tsx" => tree_sitter_typescript::LANGUAGE_TSX.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-xml")]
        "xml" => tree_sitter_xml::LANGUAGE_XML.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-yaml")]
        "yaml" => tree_sitter_yaml::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-zig")]
        "zig" => tree_sitter_zig::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-md")]
        "markdown_inline" => tree_sitter_md::INLINE_LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-regex")]
        "regex" => tree_sitter_regex::LANGUAGE.into_raw() as *const (),
        #[cfg(feature = "tree-sitter-jsdoc")]
        "jsdoc" => tree_sitter_jsdoc::LANGUAGE.into_raw() as *const (),
        _ => std::ptr::null(),
    };
    grammar.cast_mut().cast()
}
