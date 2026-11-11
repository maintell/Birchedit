//! The grammars a build without them in the binary loads from `birchpad-grammars` (ADR 0070).
//!
//! Two shapes are looked for, in this order:
//!
//! 1. **A pack of one language**: `<id>.dll`, `<id>.so` or `<id>.dylib` in the `grammars`
//!    folder of the data folder (set by the application) or next to the executable, built
//!    from `birchpad-grammars` with one `lang-*` feature. A user who needs one more language
//!    puts it there.
//! 2. **The library of all of them**: `birchpad_grammars.dll` and its names on the other
//!    platforms, next to the executable.
//!
//! A library is opened once, when a language first needs a grammar, and stays open: the
//! grammars' functions point into it.

// Loading a library and calling the C functions it exports is unsafe by nature; every block
// below says why what it does is sound.
#![allow(
    unsafe_code,
    reason = "a library is loaded and its C functions are called, each block saying why it is sound"
)]

use std::collections::HashMap;
use std::ffi::{CString, c_char, c_void};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

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

/// The folder the application keeps grammar packs in, `data/grammars` as a rule; set once, at
/// startup, before the first document is parsed.
static PACKS_FOLDER: OnceLock<PathBuf> = OnceLock::new();

/// Where the application keeps the packs a user put there.
pub fn set_pack_folder(folder: PathBuf) {
    PACKS_FOLDER.set(folder).ok();
}

/// The packs opened so far, by path; a pack stays open for the life of the process.
fn opened() -> &'static Mutex<HashMap<PathBuf, Arc<Library>>> {
    static OPENED: OnceLock<Mutex<HashMap<PathBuf, Arc<Library>>>> = OnceLock::new();
    OPENED.get_or_init(|| Mutex::new(HashMap::new()))
}

/// The name of the library of every grammar, on this platform, as cargo builds a `cdylib`.
pub(crate) const fn file_name() -> &'static str {
    if cfg!(windows) {
        "birchpad_grammars.dll"
    } else if cfg!(target_os = "macos") {
        "libbirchpad_grammars.dylib"
    } else {
        "libbirchpad_grammars.so"
    }
}

/// The name of the pack of one language: its id and the platform's library suffix.
fn pack_name(id: &str) -> String {
    format!("{id}{}", std::env::consts::DLL_SUFFIX)
}

/// The folders packs are looked for in: the application's, then the executable's.
fn folders() -> Vec<PathBuf> {
    let mut folders = Vec::new();
    if let Some(folder) = PACKS_FOLDER.get() {
        folders.push(folder.clone());
    }
    if let Ok(executable) = std::env::current_exe()
        && let Some(folder) = executable.parent()
    {
        folders.push(folder.join("grammars"));
        // The executable's own folder too: a portable copy may keep the pack beside it.
        folders.push(folder.to_path_buf());
    }
    folders
}

fn open(path: &Path) -> Result<Arc<Library>, String> {
    let mut opened = opened()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(library) = opened.get(path) {
        return Ok(library.clone());
    }
    // SAFETY: the library is one this project builds; its symbols are used as declared below.
    let library = unsafe { libloading::Library::new(path) }
        .map_err(|error| format!("cannot load the grammars from {}: {error}", path.display()))?;
    // SAFETY: the symbol is the function this project's `birchpad-grammars` exports.
    let lookup: Lookup = unsafe {
        *library
            .get::<Lookup>(b"birchpad_grammar\0")
            .map_err(|error| format!("{} has no birchpad_grammar: {error}", path.display()))?
    };
    let library = Arc::new(Library {
        lookup,
        path: path.to_path_buf(),
        _library: library,
    });
    opened.insert(path.to_path_buf(), library.clone());
    Ok(library)
}

/// The pack of `id`, if one is there.
fn pack(id: &str) -> Option<Arc<Library>> {
    let name = pack_name(id);
    for folder in folders() {
        let path = folder.join(&name);
        if !path.is_file() {
            continue;
        }
        match open(&path) {
            Ok(library) => return Some(library),
            Err(error) => eprintln!("{error}; {id} opens as plain text"),
        }
    }
    None
}

/// The library of every grammar, next to the executable, opened once.
fn all() -> Option<&'static Library> {
    static ALL: OnceLock<Option<Arc<Library>>> = OnceLock::new();
    ALL.get_or_init(|| {
        let path = std::env::current_exe()
            .ok()
            .and_then(|executable| executable.parent().map(|folder| folder.join(file_name())));
        let Some(path) = path else {
            eprintln!("the executable has no folder to load the grammars from");
            return None;
        };
        match open(&path) {
            Ok(library) => Some(library),
            Err(error) => {
                eprintln!("{error}; syntax highlighting is off");
                None
            }
        }
    })
    .as_deref()
}

/// The grammar `id`, from its pack, or from the library of all of them.
pub(crate) fn grammar(id: &str) -> Result<Language, String> {
    if let Some(library) = pack(id) {
        match language_of(&library, id) {
            Ok(language) => return Ok(language),
            // A pack that does not hold it is not a reason to stop: the whole library may.
            Err(error) => eprintln!("{error}; trying the library of every grammar"),
        }
    }
    let library = all().ok_or_else(|| {
        format!(
            "the grammars are not loaded ({} or {}/{} is missing); {id} opens as plain text",
            file_name(),
            "grammars",
            pack_name(id)
        )
    })?;
    language_of(library, id)
}

/// What `library` answers for `id`, as a tree-sitter language.
fn language_of(library: &Library, id: &str) -> Result<Language, String> {
    let name = id.to_owned();
    let id = CString::new(id).map_err(|error| error.to_string())?;
    // SAFETY: a null-terminated id, and a function this project's library exports.
    let address = unsafe { (library.lookup)(id.as_ptr()) };
    if address.is_null() {
        return Err(format!(
            "{} has no grammar for {name}",
            library.path.display()
        ));
    }
    // SAFETY: the library answered with a grammar function, which has this one type.
    let function: unsafe extern "C" fn() -> *const () = unsafe { std::mem::transmute(address) };
    // SAFETY: the function comes from a grammar crate built against the same tree-sitter.
    Ok(Language::new(unsafe { LanguageFn::from_raw(function) }))
}
