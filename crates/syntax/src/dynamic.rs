//! The grammars a build without them in the binary loads from `birchpad-grammars` (ADR 0070).
//!
//! The library sits next to the executable, whatever the platform's name for it is, and
//! answers with a grammar's C function by the language's id. It is opened once, on the first
//! language that needs a grammar, and stays open: the grammars' functions point into it.

// Loading a library and calling the C functions it exports is unsafe by nature; every block
// below says why what it does is sound.
#![allow(
    unsafe_code,
    reason = "a library is loaded and its C functions are called, each block saying why it is sound"
)]

use std::ffi::{CString, c_char, c_void};
use std::path::PathBuf;
use std::sync::OnceLock;

use tree_sitter::Language;
use tree_sitter_language::LanguageFn;

/// What `birchpad_grammar` does: a language id in, the grammar's function out, or null.
type Lookup = unsafe extern "C" fn(*const c_char) -> *mut c_void;

struct Library {
    lookup: Lookup,
    path: PathBuf,
    /// The grammars' code lives here; the library must not be closed while a grammar is used.
    _library: libloading::Library,
}

/// The library, opened once; `None` once it could not be, which is said once.
fn library() -> Option<&'static Library> {
    static LIBRARY: OnceLock<Option<Library>> = OnceLock::new();
    LIBRARY
        .get_or_init(|| match open() {
            Ok(library) => Some(library),
            Err(error) => {
                eprintln!("{error}");
                None
            }
        })
        .as_ref()
}

/// The file name of the library on this platform, as cargo names a `cdylib`.
pub(crate) const fn file_name() -> &'static str {
    if cfg!(windows) {
        "birchpad_grammars.dll"
    } else if cfg!(target_os = "macos") {
        "libbirchpad_grammars.dylib"
    } else {
        "libbirchpad_grammars.so"
    }
}

/// Where the library is looked for: next to the executable, and in the folder it is in (a
/// macOS bundle keeps both in `Contents/MacOS`).
pub(crate) fn path() -> Result<PathBuf, String> {
    let executable = std::env::current_exe().map_err(|error| error.to_string())?;
    let folder = executable
        .parent()
        .ok_or_else(|| format!("{} has no folder", executable.display()))?;
    Ok(folder.join(file_name()))
}

fn open() -> Result<Library, String> {
    let path = path()?;
    // SAFETY: the library is one this project builds; its symbols are used as declared below.
    let library = unsafe { libloading::Library::new(&path) }.map_err(|error| {
        format!(
            "cannot load the grammars from {}: {error}; syntax highlighting is off",
            path.display()
        )
    })?;
    // SAFETY: the symbol is the function this crate's `birchpad-grammars` exports.
    let lookup: Lookup = unsafe {
        *library
            .get::<Lookup>(b"birchpad_grammar\0")
            .map_err(|error| {
                format!(
                    "{} has no birchpad_grammar: {error}; syntax highlighting is off",
                    path.display()
                )
            })?
    };
    Ok(Library {
        lookup,
        path,
        _library: library,
    })
}

/// The grammar of `id`, or why there is none.
pub(crate) fn grammar(id: &str) -> Result<Language, String> {
    let library = library().ok_or_else(|| {
        format!(
            "the grammars are not loaded ({} is missing); {id} opens as plain text",
            file_name()
        )
    })?;
    let name = id.to_owned();
    let id = CString::new(id).map_err(|error| error.to_string())?;
    // SAFETY: a null-terminated id, and a function this project's library exports.
    let address = unsafe { (library.lookup)(id.as_ptr()) };
    if address.is_null() {
        return Err(format!(
            "{} has no grammar for {name}; it opens as plain text",
            library.path.display()
        ));
    }
    // SAFETY: the library answered with a grammar function, which has this one type.
    let function: unsafe extern "C" fn() -> *const () = unsafe { std::mem::transmute(address) };
    // SAFETY: the function comes from a grammar crate built against the same tree-sitter.
    Ok(Language::new(unsafe { LanguageFn::from_raw(function) }))
}
