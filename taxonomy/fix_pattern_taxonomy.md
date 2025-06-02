## 🔧 **Fix Pattern** (11 categories)

This document describes the common code change patterns developers adopt to fix bugs in Rust interop tools.

----

### 1. **Insert Method Argument**
The method definition or invocation is modified by adding one or more parameters, typically to support new data requirements or extended functionality.

---

### 2. **Mutate Method Chain**
A method chain is modified by inserting, removing, replacing, or reordering method calls to correct logic or adapt to API changes.

---

### 3. **Insert Wrapping Method**
An existing statement is wrapped inside a new method call.

---

### 4. **Mutate Condition**
A condition is mutated by inserting, deleting, or modifying logical sub-conditions (e.g., via `&&` or `||`).

---

### 5. **Insert Match Block**
A new `match` block is introduced to implement pattern-based control flow. In some cases, it replaces an existing `if-else` chain to enable more structured branching logic.

---

### 6. **Mutate Match Arm**
A match expression is modified by inserting new arms, deleting existing ones, or updating the statements associated with specific patterns.

---

### 7. **Insert `if-else` Block**
An `if-else` block is inserted to introduce conditional execution. This includes conditional blocks or guard clauses that enforce early return.

---

### 8. **Insert Wrapping `if-else`**
An existing statement is wrapped within an `if` or `else` block to constrain its execution to specific runtime scenarios.

---

### 9. **Insert Conditional Branch**
A new conditional branch is added (e.g., `if`, `else if`, or `else`) to handle an additional case.

---

### 10. **Reorder Code Block**
An existing code block is moved to a different position to change its execution order.

---

### 11. **Insert Match Arm Clause**
A new `match` arm clause is inserted to account for previously unhandled input patterns or enum variants.
