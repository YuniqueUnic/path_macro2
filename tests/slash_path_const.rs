use path_macro2::path_const;

#[test]
fn test_slash_path_const() {
    // === 基本用法 ===
    const CONFIG_PATH: &str = path_const!(config / app.toml);
    const LIB_PATH: &str = path_const!(vendor / dll / windivert.c);
    const SINGLE_SEG: &str = path_const!(config);

    // === 带点的标识符 ===
    const DOTTED_FILE: &str = path_const!(src / lib.rs);
    const MULTIPLE_DOTS: &str = path_const!(archive / backup.tar.gz);

    // === 字符串字面量（空格、特殊字符）===
    const QUOTED_PATH: &str = path_const!("my folder" / "file name.txt");
    const MIXED_QUOTED: &str = path_const!(vendor / "include files" / windivert.h);

    // === 与 concat! 组合使用 ===
    const DEF_FLAG: &str = concat!("/DEF:", path_const!(vendor / dll / windivert.def));
    const INCLUDE_FLAG: &str = concat!("/I", path_const!(vendor / include));
    const COMPLEX_FLAG: &str = concat!("--config=", path_const!(config / app.toml));

    // === 在数组中使用 ===
    const DYNAMIC_CL_ARGS: &[&str] = &[
        "/nologo",
        "/W1",
        concat!("/I", path_const!(vendor / include)),
        concat!("/DEF:", path_const!(vendor / dll / windivert.def)),
        path_const!(vendor / dll / windivert.c),
    ];

    const FILE_LIST: &[&str] = &[
        path_const!(src / main.rs),
        path_const!(src / lib.rs),
        path_const!(tests / integration.test.rs),
    ];

    // === 平台特定断言 ===
    #[cfg(target_os = "windows")]
    {
        assert_eq!(CONFIG_PATH, "config\\app.toml");
        assert_eq!(LIB_PATH, "vendor\\dll\\windivert.c");
        assert_eq!(SINGLE_SEG, "config");
        assert_eq!(DOTTED_FILE, "src\\lib.rs");
        assert_eq!(MULTIPLE_DOTS, "archive\\backup.tar.gz");
        assert_eq!(QUOTED_PATH, "my folder\\file name.txt");
        assert_eq!(MIXED_QUOTED, "vendor\\include files\\windivert.h");
        assert_eq!(DEF_FLAG, "/DEF:vendor\\dll\\windivert.def");
        assert_eq!(INCLUDE_FLAG, "/Ivendor\\include");
        assert_eq!(COMPLEX_FLAG, "--config=config\\app.toml");
        assert_eq!(DYNAMIC_CL_ARGS[2], "/Ivendor\\include");
        assert_eq!(DYNAMIC_CL_ARGS[4], "vendor\\dll\\windivert.c");
        assert_eq!(FILE_LIST[0], "src\\main.rs");
        assert_eq!(FILE_LIST[2], "tests\\integration.test.rs");
    }

    #[cfg(not(target_os = "windows"))]
    {
        assert_eq!(CONFIG_PATH, "config/app.toml");
        assert_eq!(LIB_PATH, "vendor/dll/windivert.c");
        assert_eq!(SINGLE_SEG, "config");
        assert_eq!(DOTTED_FILE, "src/lib.rs");
        assert_eq!(MULTIPLE_DOTS, "archive/backup.tar.gz");
        assert_eq!(QUOTED_PATH, "my folder/file name.txt");
        assert_eq!(MIXED_QUOTED, "vendor/include files/windivert.h");
        assert_eq!(DEF_FLAG, "/DEF:vendor/dll/windivert.def");
        assert_eq!(INCLUDE_FLAG, "/Ivendor/include");
        assert_eq!(COMPLEX_FLAG, "--config=config/app.toml");
        assert_eq!(DYNAMIC_CL_ARGS[2], "/Ivendor/include");
        assert_eq!(DYNAMIC_CL_ARGS[4], "vendor/dll/windivert.c");
        assert_eq!(FILE_LIST[0], "src/main.rs");
        assert_eq!(FILE_LIST[2], "tests/integration.test.rs");
    }

    // === 打印输出 ===
    println!("\n=== Slash-separated paths ===");
    println!("CONFIG_PATH:    {}", CONFIG_PATH);
    println!("LIB_PATH:       {}", LIB_PATH);
    println!("SINGLE_SEG:     {}", SINGLE_SEG);
    println!("DOTTED_FILE:    {}", DOTTED_FILE);
    println!("MULTIPLE_DOTS:  {}", MULTIPLE_DOTS);
    println!("QUOTED_PATH:    {}", QUOTED_PATH);
    println!("MIXED_QUOTED:   {}", MIXED_QUOTED);

    println!("\n=== With concat! ===");
    println!("DEF_FLAG:       {}", DEF_FLAG);
    println!("INCLUDE_FLAG:   {}", INCLUDE_FLAG);
    println!("COMPLEX_FLAG:   {}", COMPLEX_FLAG);

    println!("\n=== In arrays ===");
    println!("DYNAMIC_CL_ARGS: {:#?}", DYNAMIC_CL_ARGS);
    println!("FILE_LIST:       {:#?}", FILE_LIST);

    println!("\n✓ All slash-separated tests passed!");
}
