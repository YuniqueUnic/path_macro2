use path_macro2::path;

#[test]
fn test_basic_slash_paths() {
    let a = path!(vendor / include);
    let b = path!(vendor / dll / "windivert.c");
    let c = path!(vendor / dll / windivert.d);

    #[cfg(target_os = "windows")]
    {
        assert_eq!(a.to_string_lossy(), "vendor\\include");
        assert_eq!(b.to_string_lossy(), "vendor\\dll\\windivert.c");
        assert_eq!(c.to_string_lossy(), "vendor\\dll\\windivert.d");
    }

    #[cfg(not(target_os = "windows"))]
    {
        assert_eq!(a.to_string_lossy(), "vendor/include");
        assert_eq!(b.to_string_lossy(), "vendor/dll/windivert.c");
        assert_eq!(c.to_string_lossy(), "vendor/dll/windivert.d");
    }
}

#[test]
fn test_slash_with_spaces_and_quotes() {
    let g = path!("my folder" / "sub folder" / file.txt);

    #[cfg(target_os = "windows")]
    assert_eq!(g.to_string_lossy(), "my folder\\sub folder\\file.txt");

    #[cfg(not(target_os = "windows"))]
    assert_eq!(g.to_string_lossy(), "my folder/sub folder/file.txt");
}

#[test]
fn test_slash_with_variables() {
    let base = "my_base";
    let h = path!({ base } / sub / file.txt);
    let s = h.to_string_lossy();
    assert!(s.ends_with("sub/file.txt") || s.ends_with("sub\\file.txt"));
}

#[test]
fn test_slash_mixed() {
    let vendor_path = "vendor";
    let i = path!({ vendor_path } / dll / "windivert.c");
    let root = String::from("root");
    let j = path!({ root } / { format!("sub_{}", 1) } / file.txt);

    let i_s = i.to_string_lossy();
    let j_s = j.to_string_lossy();

    assert!(i_s.contains("dll"));
    assert!(i_s.ends_with("windivert.c"));
    assert!(j_s.contains("sub_1"));
    assert!(j_s.ends_with("file.txt"));
}

#[test]
fn test_unix_absolute_slash() {
    // Unix-style absolute paths
    let abs = path!("/" / "test" / "data" / "windivert.c");
    assert_eq!(abs, std::path::PathBuf::from("/test/data/windivert.c"));

    // Ensure normalization
    assert!(!abs.to_string_lossy().contains("/./"));
}

#[cfg(target_os = "windows")]
#[test]
fn test_windows_absolute_slash() {
    // Edge case: Windows drive letter path
    let a = path!("C:\\" / "bai du.txt" / windivert.c);
    assert_eq!(a.to_string_lossy(), "C:\\bai du.txt\\windivert.c",);

    // UNC-style path
    let unc = path!("\\\\server" / "share dir" / "file.txt");
    assert_eq!(unc.to_string_lossy(), "\\\\server\\share dir\\file.txt");
}
