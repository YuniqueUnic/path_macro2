use path_macro2::path;

#[test]
fn test_basic_comma_paths() {
    let a = path!(vendor, include);
    let b = path!(vendor, dll, "windivert.c");
    let c = path!(vendor, dll, windivert.d);
    let d = path!(vendor, "bai du.txt", windivert.c);

    #[cfg(target_os = "windows")]
    {
        assert_eq!(a.to_string_lossy(), "vendor\\include");
        assert_eq!(b.to_string_lossy(), "vendor\\dll\\windivert.c");
        assert_eq!(c.to_string_lossy(), "vendor\\dll\\windivert.d");
        assert_eq!(d.to_string_lossy(), "vendor\\bai du.txt\\windivert.c");
    }

    #[cfg(not(target_os = "windows"))]
    {
        assert_eq!(a.to_string_lossy(), "vendor/include");
        assert_eq!(b.to_string_lossy(), "vendor/dll/windivert.c");
        assert_eq!(c.to_string_lossy(), "vendor/dll/windivert.d");
        assert_eq!(d.to_string_lossy(), "vendor/bai du.txt/windivert.c");
    }
}

#[test]
fn test_comma_with_variables() {
    let base = "vendor";
    let p = path!({ base }, dll, file.txt);
    let s = p.to_string_lossy();
    assert!(s.ends_with("dll/file.txt") || s.ends_with("dll\\file.txt"));
}

#[test]
fn test_comma_with_expressions() {
    let root = String::from("root");
    let sub = 2;
    let p = path!({ root }, { format!("sub_{}", sub) }, file.txt);
    let s = p.to_string_lossy();
    assert!(s.contains("sub_2"));
    assert!(s.ends_with("file.txt"));
}

#[test]
fn test_unix_absolute_slash() {
    // Unix-style absolute paths
    let abs = path!("/", "test", "data", "windivert.c");
    assert_eq!(abs, std::path::PathBuf::from("/test/data/windivert.c"));

    // Ensure normalization
    assert!(!abs.to_string_lossy().contains("/./"));
}

#[cfg(target_os = "windows")]
#[test]
fn test_windows_absolute_slash() {
    // Edge case: Windows drive letter path
    let a = path!("C:\\", "bai du.txt", windivert.c);
    assert_eq!(a.to_string_lossy(), "C:\\bai du.txt\\windivert.c",);

    // UNC-style path
    let unc = path!("\\\\server", "share dir", "file.txt");
    assert_eq!(unc.to_string_lossy(), "\\\\server\\share dir\\file.txt");
}
