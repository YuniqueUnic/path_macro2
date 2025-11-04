#![doc = include_str!("../README.md")]

/// Cross-platform path construction macro.
///
/// Returns a [`PathBuf`].
///
/// # Supported Syntax
///
/// Supports two styles of separators:
/// - `path!(a / b / c)` — uses slashes (`/`)
/// - `path!(a, b, c)` — uses commas (`,`).
///
/// # Supported Segment Types
///
/// - **Identifiers:** `vendor`, `dll` (converted with `stringify!`)
/// - **Dotted identifiers:** `file.txt`, `windivert.c` (treated as single segments)
/// - **String literals:** `"my folder"`, `"file name.txt"`
/// - **Variable expressions:** wrapped in curly braces `{base_path}`, `{my_var}`
///
/// # Examples
///
/// ```rust
/// use path_macro2::path;
///
/// // Basic usage
/// let path1 = path!(vendor / dll / windivert.c);
/// let path2 = path!(vendor, dll, windivert.c);
///
/// // Quoted segments (for names containing spaces)
/// let path3 = path!("my folder" / "sub folder" / file.txt);
///
/// // Using variables (wrapped in `{}`)
/// let base = "vendor";
/// let path4 = path!({base} / dll / file.txt);
///
/// // ---
/// // Platform-specific examples
///
/// // Unix absolute path
/// #[cfg(not(target_os = "windows"))]
/// {
///     let abs = path!("/", "test", "data", "windivert.c");
///     assert_eq!(abs, std::path::PathBuf::from("/test/data/windivert.c"));
/// }
///
/// // Windows absolute path (with drive letter)
/// #[cfg(target_os = "windows")]
/// {
///     let a = path!("C:\\", "Program Files", "Windivert", "driver.sys");
///     assert_eq!(
///         a.to_string_lossy(),
///         "C:\\Program Files\\Windivert\\driver.sys"
///     );
///     // UNC-style path
///     let unc = path!("\\\\server", "share dir", "file.txt");
///     assert_eq!(unc.to_string_lossy(), "\\\\server\\share dir\\file.txt");
/// }
/// ```
///
/// Works consistently across all platforms.
#[macro_export]
macro_rules! path {
    // === Phase 1: Build segments (accumulate tokens until a delimiter is found) ===

    // When encountering a string literal, treat it as a complete segment
    (@build_seg [$($result:expr),*] [$($current:tt)*] $lit:literal $($rest:tt)*) => {
        path!(@build_seg [$($result,)* path!(@finish_seg [$($current)*]), $lit.to_string()] [] $($rest)*)
    };

    // When encountering a variable expression {expr}, treat it as a complete segment
    (@build_seg [$($result:expr),*] [$($current:tt)*] { $($expr:tt)+ } $($rest:tt)*) => {
        path!(@build_seg [$($result,)* path!(@finish_seg [$($current)*]), ($($expr)+).to_string()] [] $($rest)*)
    };

    // When encountering a slash `/`, complete the current segment
    (@build_seg [$($result:expr),*] [$($current:tt)+] / $($rest:tt)*) => {
        path!(@build_seg [$($result,)* path!(@finish_seg [$($current)+])] [] $($rest)*)
    };

    // When encountering a slash `/` but the current segment is empty, skip it
    (@build_seg [$($result:expr),*] [] / $($rest:tt)*) => {
        path!(@build_seg [$($result),*] [] $($rest)*)
    };

    // When encountering a comma `,`, complete the current segment
    (@build_seg [$($result:expr),*] [$($current:tt)+] , $($rest:tt)*) => {
        path!(@build_seg [$($result,)* path!(@finish_seg [$($current)+])] [] $($rest)*)
    };

    // When encountering a comma `,` but the current segment is empty, skip it
    (@build_seg [$($result:expr),*] [] , $($rest:tt)*) => {
        path!(@build_seg [$($result),*] [] $($rest)*)
    };

    // Accumulate normal tokens into the current segment
    (@build_seg [$($result:expr),*] [$($current:tt)*] $next:tt $($rest:tt)*) => {
        path!(@build_seg [$($result),*] [$($current)* $next] $($rest)*)
    };

    // End of tokens: process the final segment (if any)
    (@build_seg [$($result:expr),*] [$($current:tt)+]) => {
        vec![$($result,)* path!(@finish_seg [$($current)+])]
    };

    // End of tokens: no remaining segment
    (@build_seg [$($result:expr),*] []) => {
        vec![$($result),*]
    };

    // === Helper: finalize one segment (stringify or return empty) ===
    (@finish_seg []) => {
        String::new()
    };

    (@finish_seg [$($tokens:tt)+]) => {
        stringify!($($tokens)+).to_string()
    };

    // === Entry point ===
    ($($tokens:tt)*) => {{
        let segments: Vec<String> = path!(@build_seg [] [] $($tokens)*);
        let mut path = std::path::PathBuf::new();
        for seg in segments {
            if !seg.is_empty() {
                path.push(seg);
            }
        }
        path
    }};
}
