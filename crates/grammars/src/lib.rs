//! Birchpad's tree-sitter grammars in a library of their own (ADR 0070).
//!
//! The application's release builds do not link the grammars: the parser tables are 50 MB of
//! the executable, and only a few languages are used in a session. The executable loads this
//! library from its own folder and asks it for a language by id.

#![allow(
    unsafe_code,
    reason = "the one exported function is C ABI: a null-terminated id in, a function pointer \
              to the grammar out, which is how tree-sitter's grammars are handed over"
)]

use std::ffi::{CStr, c_char, c_void};

/// The function that returns the grammar of `id`, a Birchpad language id, as a function
/// pointer; null when there is no such language.
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
        "asm" => tree_sitter_asm::LANGUAGE.into_raw() as *const (),
        "bash" => tree_sitter_bash::LANGUAGE.into_raw() as *const (),
        "batch" => tree_sitter_batch::LANGUAGE.into_raw() as *const (),
        "c" => tree_sitter_c::LANGUAGE.into_raw() as *const (),
        "cs" => tree_sitter_c_sharp::LANGUAGE.into_raw() as *const (),
        "cpp" => tree_sitter_cpp::LANGUAGE.into_raw() as *const (),
        "cmake" => tree_sitter_cmake::LANGUAGE.into_raw() as *const (),
        "css" => tree_sitter_css::LANGUAGE.into_raw() as *const (),
        "dart" => tree_sitter_dart::LANGUAGE.into_raw() as *const (),
        "diff" => tree_sitter_diff::LANGUAGE.into_raw() as *const (),
        "dockerfile" => tree_sitter_containerfile::LANGUAGE.into_raw() as *const (),
        "elixir" => tree_sitter_elixir::LANGUAGE.into_raw() as *const (),
        "erlang" => tree_sitter_erlang::LANGUAGE.into_raw() as *const (),
        "go" => tree_sitter_go::LANGUAGE.into_raw() as *const (),
        "graphql" => tree_sitter_graphql::LANGUAGE.into_raw() as *const (),
        "groovy" => tree_sitter_groovy::LANGUAGE.into_raw() as *const (),
        "hcl" => tree_sitter_hcl::LANGUAGE.into_raw() as *const (),
        "html" => tree_sitter_html::LANGUAGE.into_raw() as *const (),
        "ini" => tree_sitter_ini::LANGUAGE.into_raw() as *const (),
        "java" => tree_sitter_java::LANGUAGE.into_raw() as *const (),
        "javascript" => tree_sitter_javascript::LANGUAGE.into_raw() as *const (),
        "json" => tree_sitter_json::LANGUAGE.into_raw() as *const (),
        "kotlin" => tree_sitter_kotlin_sg::LANGUAGE.into_raw() as *const (),
        "lua" => tree_sitter_lua::LANGUAGE.into_raw() as *const (),
        "makefile" => tree_sitter_make::LANGUAGE.into_raw() as *const (),
        "markdown" => tree_sitter_md::LANGUAGE.into_raw() as *const (),
        "nginx" => tree_sitter_nginx::LANGUAGE.into_raw() as *const (),
        "nix" => tree_sitter_nix::LANGUAGE.into_raw() as *const (),
        "objc" => tree_sitter_objc::LANGUAGE.into_raw() as *const (),
        "pascal" => tree_sitter_pascal::LANGUAGE.into_raw() as *const (),
        "plantuml" => birchpad_tree_sitter_plantuml::LANGUAGE.into_raw() as *const (),
        "powershell" => tree_sitter_powershell::LANGUAGE.into_raw() as *const (),
        "properties" => tree_sitter_properties::LANGUAGE.into_raw() as *const (),
        "proto" => tree_sitter_proto::LANGUAGE.into_raw() as *const (),
        "python" => tree_sitter_python::LANGUAGE.into_raw() as *const (),
        "r" => tree_sitter_r::LANGUAGE.into_raw() as *const (),
        "ruby" => tree_sitter_ruby::LANGUAGE.into_raw() as *const (),
        "rust" => tree_sitter_rust::LANGUAGE.into_raw() as *const (),
        "solidity" => tree_sitter_solidity::LANGUAGE.into_raw() as *const (),
        "sql" => tree_sitter_sequel::LANGUAGE.into_raw() as *const (),
        "svelte" => tree_sitter_svelte_ng::LANGUAGE.into_raw() as *const (),
        "swift" => tree_sitter_swift::LANGUAGE.into_raw() as *const (),
        "toml" => tree_sitter_toml_ng::LANGUAGE.into_raw() as *const (),
        "typescript" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into_raw() as *const (),
        "tsx" => tree_sitter_typescript::LANGUAGE_TSX.into_raw() as *const (),
        "xml" => tree_sitter_xml::LANGUAGE_XML.into_raw() as *const (),
        "yaml" => tree_sitter_yaml::LANGUAGE.into_raw() as *const (),
        "zig" => tree_sitter_zig::LANGUAGE.into_raw() as *const (),
        "markdown_inline" => tree_sitter_md::INLINE_LANGUAGE.into_raw() as *const (),
        "regex" => tree_sitter_regex::LANGUAGE.into_raw() as *const (),
        "jsdoc" => tree_sitter_jsdoc::LANGUAGE.into_raw() as *const (),
        _ => std::ptr::null(),
    };
    grammar.cast_mut().cast()
}
