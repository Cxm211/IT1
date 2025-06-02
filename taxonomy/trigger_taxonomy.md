## ⚠️ **Trigger** (25 categories)


This taxonomy includes 25 recurring trigger types extracted from real-world issues reported in Bindgen, Cbindgen, and CXX. Each trigger is associated with code constructs or tool configuration options that cause interoperability failures between Rust and C/C++.


### 1. **Struct**
Complex structures containing nested fields, bitfields, flexible arrays, or alignment attributes.

### 2. **Tool Configuration Option**
Command-line flags,  code annotations, or configuration file settings used to customize the generated code trigger the issues. 

### 3. **Primitive Datatype**
Primitive data types (e.g., `uint64_t`, `char`, `size_t`) causing mismatches or incorrect translations.

### 4. **Function**
Function declarations or definitions, including complex signatures, return types, or callbacks.

### 5. **Template**
C++ templates that cannot be correctly expanded or mapped.

### 6. **Class**
C++ class definitions or inheritance hierarchies that are not fully supported or translated.

### 7. **Type Alias**
Use of `typedef`, `using`, or Rust’s `type` keyword causing issues.

### 8. **Enum**
Enumeration definitions, especially generic enums like `Option<T>` and `Result<T, E>` in Rust.

### 9. **Attribute**
Annotations such as `#[repr(transparent)]`, `__attribute__((packed))`, or `#[namespace = "..."]`.

### 10. **Pointer**
Raw or smart pointers (`*const T`, `Box<T>`, `rust::Box<T>`) used in arguments, return types, or structs.

### 11. **Bitfield**
Bitfield definitions within `struct`s that result in layout and alignment problems.

### 12. **Union**
Union in the input that cause errors.

### 13. **Array**
Fixed-size arrays in the input that cause errors.

### 14. **Builtin Type**
To support complex data type mappings, besides primitive data types, CXX provides a set of built-in types that serve as bridges when a native equivalent is missing in one language. Built-in types involved in these issues include, for example, `rust::Vec<T>` on the C++ side, which provides a safe interface to Rust’s `Vec<T>`, and `rust::Box<T>`, corresponding to `Box<T>` in Rust, which facilitates cross-language interaction with heap-allocated values managed by Rust.

### 15. **Comment**
Code comments (e.g., markdown comments) that interfere with parsing or generate unexpected output.

### 16. **Namespace**
C++ namespaces or CXX-specific `#[namespace = "..."]` directives in the input causing issues.

### 17. **Reference**
Use of references (`&T`, `&mut T`) in the input that cause errors.

### 18. **Macro**
C/C++ preprocessor macros in the input headers processed by Bindgen, or to the built-in macros defined by the CXX crate, either of which triggers the reported issues.

### 19. **Constant**
Constants declared with the `const` keyword in the input that cause errors.

### 20. **Lifetime**
Rust lifetime annotations used in CXX bridge modules, which ensure that all borrows crossing the FFI boundary remain valid and safe, in the input that cause errors.

### 21. **Module Path**
It refers to the internal organization of Rust modules, including both inline modules defined in the same file and submodules declared across files and directories.

### 22. **Vector**
Refers to the `std::vector` type from the C++ standard library, which Bindgen translates into Rust FFI bindings without preserving its semantic implementation, or the `Vec<T>` type in Rust, which interoperates with the built-in type `rust::Vec<T>` safely in the FFI interface for CXX.

### 23. **Target Platform**
 Issues that arise only when building for a specific target platform or architecture (e.g., using `cargo --target armv7-linux-androideabi`).

### 24. **Dependent Crate**
It refers to external libraries explicitly declared as dependencies in the project's configuration files. This type of trigger often leads to parsing errors when the Cbindgen configuration option `parse_deps` is enabled, as the tool attempts to analyze dependent crates. 

### 25. **Flexible Array Member**
The flexible array member typically appears as the final field in a struct, and its incomplete size definition makes it error-prone.

---

**Coverage Note**: These 25 trigger types account for at least 92% of reported interop tool issues across the three tools.
