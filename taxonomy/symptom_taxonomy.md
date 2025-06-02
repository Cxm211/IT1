## ⚠️ Symptom (5 types)

- **Crash**  
  The tool terminates unexpectedly while generating bindings or expanding macros (only in CXX).

- **Hang**  
  tool hanging during execution, requiring a manual termination.

- **Uncompilable code**  
  The generated bindings are uncompilable when using Bindgen or Cbindgen as a standalone program, or will cause the project to fail to build when the tool is used as a library within a Rust project.

- **Functionality error**  
  The generated code compiles successfully but does not preserve the original semantics or behavior.

- **Performance degradation**  
  The tool exhibits significantly slower execution time or startup.
