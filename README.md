# path_macro2

[![Crates.io](https://img.shields.io/crates/v/path_macro2.svg)](https://crates.io/crates/path_macro2)
[![Documentation](https://docs.rs/path_macro2/badge.svg)](https://docs.rs/path_macro2)
[![License](https://img.shields.io/crates/l/path_macro2)](https://github.com/yuniqueunic/path_macro2#license)
![Crates.io Total Downloads](https://img.shields.io/crates/d/path_macro2)
![Deps.rs Crate Dependencies (latest)](https://img.shields.io/deps-rs/path_macro2/latest)

A cross-platform path construction macro for Rust that provides an intuitive
syntax for building file paths while automatically handling platform-specific
path separators.

## Features

- **Dual syntax support**: Use either slash (`/`) or comma (`,`) separators
- **Cross-platform**: Automatically uses correct path separators (`\` on
  Windows, `/` on Unix-like systems)
- **Runtime and compile-time**: `path!` for runtime with variable interpolation,
  `path_const!` for compile-time constants
- **Multiple segment types**: Identifiers, dotted names, string literals, and
  expressions
- **Zero dependencies**: Lightweight macro-only implementation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
path_macro2 = "0.1.2"
```

## Usage

### Runtime Path Construction with `path!`

The `path!` macro creates `PathBuf` instances at runtime and supports variable
interpolation.

#### Basic Syntax

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

### Compile-Time Path Constants with `path_const!`

The `path_const!` macro generates compile-time string constants, perfect for use
with `concat!` and in `const` contexts.

#### Basic Usage

```rust
use path_macro2::path_const;

// Compile-time constants
const CONFIG_PATH: &str = path_const!(config / app.toml);
const LIB_PATH: &str = path_const!(vendor / dll / windivert.c);

// Results:
// Windows: "config\\app.toml", "vendor\\dll\\windivert.c"
// Unix:    "config/app.toml",  "vendor/dll/windivert.c"
```

#### Combining with `concat!` for Build Flags

```rust
use path_macro2::path_const;

// Perfect for build scripts and compiler flags
const DEF_FLAG: &str = concat!("/DEF:", path_const!(vendor / dll / windivert.def));
const INCLUDE_FLAG: &str = concat!("/I", path_const!(vendor / include));

// Use in arrays
const BUILD_ARGS: &[&str] = &[
    "/nologo",
    "/W1",
    concat!("/I", path_const!(vendor / include)),
    concat!("/DEF:", path_const!(vendor / dll / windivert.def)),
    path_const!(vendor / dll / windivert.c),
];
```

#### String Literals and Dotted Identifiers

```rust
use path_macro2::path_const;

// Handles spaces and special characters
const DOC_PATH: &str = path_const!("my folder" / "sub folder" / file.txt);
const MIXED_PATH: &str = path_const!(vendor / "include files" / windivert.h);

// Dotted identifiers work seamlessly
const SOURCE_FILE: &str = path_const!(src / lib.rs);
const ARCHIVE: &str = path_const!(backup / data.tar.gz);
```

### Complex Examples

#### Runtime Path Construction

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

#### Build Script with Compile-Time Paths

```rust
use path_macro2::path_const;

// build.rs
const DYNAMIC_CL_ARGS: &[&str] = &[
    concat!("/I", path_const!(vendor / include)),
    "/ZI",
    "/JMC",
    "/nologo",
    "/TC",
    "/FC",
    "/errorReport:queue",
    path_const!(vendor / dll / windivert.c),
    "/link",
    "advapi32.lib",
    "/NODEFAULTLIB",
    concat!("/DEF:", path_const!(vendor / dll / windivert.def)),
    "/MANIFEST",
    "/DLL",
];

fn main() {
    // let mut compiler = cc::Build::new().get_compiler().to_command();
    let mut command = std::process::Command::new("cl");
    for &flag in DYNAMIC_CL_ARGS {
        // compiler.arg(flag);
        command.arg(flag);
    }
    // ... rest of build logic
}
```

## How It Works

### `path!` Macro (Runtime)

The `path!` macro processes path segments and automatically:

1. **Converts identifiers to strings**: `vendor` becomes `"vendor"`
2. **Handles dotted identifiers**: `file.txt` becomes `"file.txt"`
3. **Preserves string literals**: `"my folder"` stays as-is
4. **Evaluates expressions**: `{base_path}` evaluates the variable
5. **Builds PathBuf**: Uses `std::path::PathBuf::push()` for proper platform
   handling

The result is always a `std::path::PathBuf` that uses the correct path
separators for the target platform.

### `path_const!` Macro (Compile-Time)

The `path_const!` macro generates compile-time string constants:

1. **Converts identifiers to strings**: `vendor` becomes `"vendor"`
2. **Handles dotted identifiers**: `file.txt` becomes `"file.txt"`
3. **Preserves string literals**: `"my folder"` stays as-is
4. **Joins with platform separators**: Uses `concat!` for zero-runtime-cost
5. **No variable support**: Only literals and identifiers (use `path!` for
   variables)

The result is always a `&'static str` with platform-appropriate separators.

## Comparison with Alternatives

| Method                     | Cross-platform | Readable | Variables | Compile-time | Runtime |
| -------------------------- | -------------- | -------- | --------- | ------------ | ------- |
| `path_macro2::path!`       | ✅             | ✅       | ✅        | ❌           | ✅      |
| `path_macro2::path_const!` | ✅             | ✅       | ❌        | ✅           | ✅      |
| `std::path::Path::join()`  | ✅             | ❌       | ✅        | ❌           | ✅      |
| String concatenation       | ❌             | ❌       | ✅        | ✅           | ✅      |
| `format!()` with `/`       | ❌             | ⚠️       | ✅        | ❌           | ✅      |
| `concat!()` with `/`       | ❌             | ⚠️       | ❌        | ✅           | ✅      |

## When to Use Which Macro

- **Use `path!`** when you need:
  - Runtime path construction
  - Variable interpolation
  - `PathBuf` instances
  - Dynamic path building

- **Use `path_const!`** when you need:
  - Compile-time constants
  - Build script configurations
  - Static path definitions
  - Integration with `concat!` macro
  - Zero runtime overhead

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

```
```
