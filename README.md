# 🧵 Fortifying the Seams Between C/C++ and Rust: Dataset of Interop Tool Bugs

This repository contains the dataset and replication package accompanying our paper:

**Fortifying the Seams Between C/C++ and Rust: Characterizing Bugs in Interop Tools**  


---

## 📦 Dataset Overview

We systematically analyze **320 real-world bugs** from three widely-used Rust interop tools:

| Tool       |  #Closed | #Open | #Total Issue |
|------------|----------------|------|--------|
| Bindgen    | 143            | 94   | 237    |
| Cbindgen   | 35             | 24   | 59     |
| CXX        | 22             | 2    | 24     |
| **Total**  | **120**        |  **200**  |   **320**     |

## 📁 Repository Structure
- [`taxonomy`](./taxonomy/) – Detailed taxonomy of symptoms, causes, triggers, and fix patterns.

- [`labelling_bugs`](./labelling_bugs/) – Final labels for all 320 issues across tools in both `xlsx` and `csv` format.  
  Each entry includes:
  - **ID**: A unique identifier defined in our dataset.
  - **Status**: Whether the issue is `open` or `closed` at the point of collection (Feb 2025).
  - **Title**: Title of the GitHub issue.
  - **Issue Link**: Direct URL to the original GitHub issue report.
  - **PR or Commit Link**: URL to the associated PR or commit, if available.
  - **Trigger**: Labels of trigger.
  - **Symptom**: Labels of symptom.
  - **Cause (Sub-category)**: Labels of cause with fine-grained cause categories (e.g., `Incorrect Mutability`, `Missing Wrapper`).
  - **Cause (High-level)**: Labels of cause with the high-level cause categories (`UFC`, `INC`, `TBE`, `TRE`, etc.).
  - **Fix Pattern**: Labels of fix pattern.
