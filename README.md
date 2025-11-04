## path_macro2

[![Crates.io](https://img.shields.io/crates/v/path_macro2.svg)](https://crates.io/crates/path_macro2)
[![Documentation](https://docs.rs/path_macro2/badge.svg)](https://docs.rs/path_macro2)
[![License](https://img.shields.io/crates/l/path_macro)](https://github.com/yuniqueunic/path_macro2#license)
![Crates.io Total Downloads](https://img.shields.io/crates/d/path_macro2)
![Deps.rs Crate Dependencies (latest)](https://img.shields.io/deps-rs/path_macro2/latest)

A cross-platform path construction macro for Rust that provides an intuitive
syntax for building file paths while automatically handling platform-specific
path separators.

### Features

- **Dual syntax support**: Use either slash (`/`) or comma (`,`) separators
- **Cross-platform**: Automatically uses correct path separators (`\` on
  Windows, `/` on Unix-like systems)
- **Variable interpolation**: Support for runtime variables and expressions
- **Multiple segment types**: Identifiers, dotted names, string literals, and
  expressions
- **Zero dependencies**: Lightweight macro-only implementation

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
path_macro2 = "0.1.2"
```

### Usage

#### Basic Syntax

The macro supports two equivalent syntaxes:

```rust
use path_macro2::path;

// Slash syntax
let path1 = path!(vendor / dll / windivert.c);

// Comma syntax  
let path2 = path!(vendor, dll, windivert.c);

// Both produce the same result:
// Windows: "vendor\dll\windivert.c"
// Unix:    "vendor/dll/windivert.c"
```

#### Supported Segment Types

##### Identifiers and Dotted Names

```rust
use path_macro2::path;
let path = path!(vendor / include);           // Simple identifiers
let file = path!(config / settings.json);    // Dotted identifiers
```

##### String Literals (for spaces and special characters)

```rust
use path_macro2::path;
let path = path!("my folder" / "sub folder" / file.txt);
let docs = path!("Program Files" / "MyApp" / readme.md);
```

##### Variable Interpolation

```rust
use path_macro2::path;
let base = "vendor";
let version = "1.0";

// Variables wrapped in curly braces
let path = path!({base} / dll / file.txt);
let versioned = path!(libs / {format!("v{}", version)} / library.so);
```

#### Platform-Specific Examples

##### Unix/Linux Absolute Paths

```rust
use path_macro2::path;
let abs_path = path!("/" / "usr" / "local" / "bin" / "myapp");
// Result: "/usr/local/bin/myapp"
```

##### Windows Paths

```rust
use path_macro2::path;
// Drive letter paths
let win_path = path!("C:\\" / "Program Files" / "MyApp" / "app.exe");
// Result: "C:\Program Files\MyApp\app.exe"

// UNC network paths
let unc_path = path!("\\\\" / "server" / "share" / "file.txt");
// Result: "\\server\share\file.txt"
```

#### Complex Examples

```rust
use path_macro2::path;

fn main() {
    let project_root = std::env::var("PROJECT_ROOT").unwrap_or_else(|_| ".".to_string());
    let build_type = "release";
    
    // Mixed usage with variables and literals
    let output_path = path!({project_root} / "target" / {build_type} / "myapp.exe");
    
    // Handling paths with spaces
    let data_path = path!({project_root} / "test data" / "sample files" / input.csv);
    
    // Cross-platform configuration
    let config_path = if cfg!(windows) {
        path!("C:\\" / "ProgramData" / "MyApp" / config.toml)
    } else {
        path!("/" / "etc" / "myapp" / config.toml)
    };
    
    println!("Output: {}", output_path.display());
    println!("Data: {}", data_path.display());
    println!("Config: {}", config_path.display());
}
```

### How It Works

The `path!` macro processes path segments and automatically:

1. **Converts identifiers to strings**: `vendor` becomes `"vendor"`
2. **Handles dotted identifiers**: `file.txt` becomes `"file.txt"`
3. **Preserves string literals**: `"my folder"` stays as-is
4. **Evaluates expressions**: `{base_path}` evaluates the variable
5. **Builds PathBuf**: Uses `std::path::PathBuf::push()` for proper platform
   handling

The result is always a `std::path::PathBuf` that uses the correct path
separators for the target platform.

### Comparison with Alternatives

| Method                    | Cross-platform | Readable | Variables | Compile-time |
| ------------------------- | -------------- | -------- | --------- | ------------ |
| `path_macro2::path!`      | ✅             | ✅       | ✅        | ✅           |
| `std::path::Path::join()` | ✅             | ❌       | ✅        | ❌           |
| String concatenation      | ❌             | ❌       | ✅        | ❌           |
| `format!()` with `/`      | ❌             | ⚠️       | ✅        | ❌           |

### License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
