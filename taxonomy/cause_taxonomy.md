
## ⚠️ **Cause** (11 categories)
### 🔍 Unfaithful Code (UFC)

These issues occur when the generated code is compilable but semantically diverges from the intent or behavior of the original code.

- **Incorrect Function Signature**  
  Function declarations in the generated bindings differ from the original signatures (e.g., missing arguments, incorrect return types).

- **Incorrect Link Name**  
  The `#[link_name]` attribute is incorrect in generated binding, leading to linker errors or incorrect symbol resolution.

- **Incorrect Data Type**  
  The data types (e.g., `size_t`) are mapped to inappropriate or unsafe data types in generated bindings.

- **Incorrect Mutability**  
  The `mut` qualifier in the original code is mapped incorrectly as `const`.

- **Incorrect Comment**  
  Comments from the original source code are missing, misplaced, or redundantly included in the generated bindings.

- **Incorrect Constant Value**  
  Constants in the generated code differ from the original values (e.g., `#define FOO 100` becomes `const FOO: i32 = 0`).

- **Incorrect Identifier**  
  Identifier names are valid but differ from the originals.

- **Incorrect Namespace**  
  Mismatch in translating C++ or Rust namespaces.

- **Incorrect Declaration Order**  
  The order of declarations in the generated code are incorrect.

-----

### 🔍 Incomplete Code (INC)

These issues arise when the generated bindings are structurally or semantically incomplete.

- **Missing Type Definition**  
  A referenced type (e.g., struct, enum) is not defined in the generated bindings within the corresponding scope.

- **Missing Function Definition**  
  A function declared in the original source is omitted in the generated bindings, making it unavailable for invocation from the target language.

- **Missing Constant Definition**  
  Required constants are omitted.

- **Missing Alias Definition**  
  Type aliases (e.g., `typedef`, `using`) are not preserved in the bindings.

- **Missing Generic Argument**  
  Generic parameters (e.g., `<T>`) for types (e.g., struct, template) are not generated.

- **Missing Lifetime Bound**  
  Rust lifetime annotations necessary for safe usage are missing.

- **Missing Wrapper**  
  Required wrappers (e.g., using `ManuallyDrop`) to safely manage cross-language data are not generated.

- **Missing Attribute**  
  Critical attributes like `#[repr(C, packed)]` are omitted.

- **Missing Macro Guards**  
  Conditional compilation guards (e.g., `#ifdef`) in C/C++ are not generated.

------
### 🔍 Tool Behavior Error (TBE)

These issues stem from incorrect or unexpected behaviors of the interop tool itself, rather than from the input code or output bindings.

- **Misbehaving Configuration Option**  
  Tool configuration options (e.g., `allowlist`, `opaque-type`, `parse_deps`) do not function as expected.

- **Unexpected Warning**  
  The tool emits compiler or runtime warnings that are unexpected or irrelevant, which may confuse users.

- **Attribute Handling Error**  
  Certain attributes such as `#![deny(missing_docs)]` are not correctly handled by the CXX bridge module.

- **Lifetime Handling Error**  
  Lifetime annotations are not correctly handled by the CXX bridge module.

- **Infinite Loop / Recursion**  
  The tool enters a non-terminating loop or recursive state during parsing or code generation.

- **Slow Execution**  
  The tool takes significantly longer than expected to run on certain inputs or during startup.

-----
### 🔍 Memory Error / Mismatch (MEM)

These issues arise when the memory layout of the generated bindings is inconsistent with the original C/C++ code or violating ABI compatibility.

- **Size Mismatch**  
  The size of a struct or type in the generated Rust bindings does not match the size in the original C/C++ definition.

- **Alignment Mismatch**  
  The alignment of a struct or type in the generated Rust bindings does not match the size in the original C/C++ definition.

- **Offset Mismatch**  
  The byte offset of elements in a struct or type in the generated Rust bindings does not match the size in the original C/C++ definition.

- **Packed and Aligned Attribute Conflict Issue**  
  The generated code contains conflicting `#[repr(packed)]` and `#[repr(align(...))]` attributes, which are not allowed in Rust.

- **Overflow Error**  
  A field or value exceeds the representable memory bounds.

----
### 🔍 Compatibility Issue (CPI)

These issues arise when the interop tool behaves incorrectly or fails entirely due to incompatibilities with the toolchain version, operating platform, or compilation target.

- **Version Compatibility Issue**  
  The tool fails or produces incorrect bindings when used with specific versions of dependencies, compilers, or itself.

- **Platform Compatibility Issue**  
  The tool fails or behaves incorrectly across operating systems(e.g., Linux vs. macOS).

- **Target Compatibility Issue**  
  The tool fails to correctly generate bindings for specific compilation targets (e.g., cross-compiling to Raspberry Pi).

-------
### 🔍 Identifier Resolution Error (IDRE)

The generated bindings fail to resolve or manage identifiers.


- **Invalid Identifier**  
  Identifier names in the bindings violate Rust naming conventions (e.g., reserved keywords used without name mangling).

- **Duplicated Definition**  
  The tool generates multiple definitions for the same identifier.

- **Alias Cycle**  
  A circular type aliasing relationship is introduced (e.g., `type A = B; type B = A;`).

- **Name Shadowing Error**  
  A generated local variable unintentionally reuses the name of an existing parameter or global constant.

----
### 🔍 Tool Runtime Error (TRE)

These issues refer to internal failures reported by the compiler or runtime during tool execution, leading to abrupt termination.

- **Panic**  
  The tool encounters an unrecoverable internal error and terminates unexpectedly, typically with a panic message and stack trace.

- **Segmentation Fault**  
  A low-level memory access violation occurs during execution.

------

### 🔍 Derived Traits Error (DTE)

These issues arise when trait derivation (`#[derive(...)]`) is missing or incorrectly applied in the generated bindings, resulting in compilation failures or semantic mismatches.

- **Missing Derived Trait**  
  Required `#[derive(...)]` annotations (e.g., `Clone`, `Debug`, `Default`) are omitted from generated structs or enums.

- **Wrong Derived Trait**  
  The tool applies inappropriate or unsupported derived traits on the generated structs or enums.

-------

### 🔍 Import Resolution Error (IMRE)

These issues occur when the generated bindings fail to correctly reference external modules, crates, or headers due to errors in import statements or module resolution.

- **Missing Import Statement**  
  Required `#include` in C/C++ or `use` in Rust statements are missing.

- **Incorrect Import Path**  
  Import, module or crate paths are incorrectly generated (e.g., wrong relative path).

- **Crate/Module Parsing Failure**  
  The CBindgen tool fails to parse external crates or modules.

---

### 🔍 Code Safety (USC)

These issues occur when the generated code violates Rust’s safety guarantees.

- **Unsafe FFI**  
  The generated code violates foreign function interface (FFI) guarantees or not safe.
---

### 🔍 Invalid or Redundant Code (IRC)

These issues where the tool generates code containing redundant elements that make the bindings invalid, leading to compilation errors. 

- **Unused Type Parameter**  
  A generic type parameter is declared but never used, causing compiler warnings or confusion.

- **Duplicated Const Qualifier**  
  The `const` qualifier is redundantly applied two times to the same data (e.g., `const const int x;`).

- **Invalid Shim Function**  
  Helper (shim) functions are incorrectly generated, which is invalid.



