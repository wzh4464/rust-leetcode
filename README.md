# Rust LeetCode Solutions

A well-organized collection of LeetCode solutions implemented in Rust with comprehensive tests and benchmarks.

## Project Structure

```
rust-leetcode/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── problems/           # Individual problem solutions
│   │   ├── mod.rs         # Problem module declarations
│   │   ├── p594_longest_harmonious_subsequence.rs
│   │   └── p3330_find_original_typed_string.rs
│   └── templates/          # Template files for new problems
│       └── problem_template.rs
├── main.rs                 # Binary entry point with examples
├── Cargo.toml             # Project configuration
└── README.md              # This file
```

## Running the Code

### Run the binary with examples:
```bash
cargo run
```

### Run all tests:
```bash
cargo test
```

### Run benchmarks:
```bash
cargo bench
```

### Run tests for a specific problem:
```bash
cargo test p594
cargo test p3330
```

## Adding New Problems

1. Copy the template from `src/templates/problem_template.rs`
2. Rename it to `src/problems/p[number]_[snake_case_title].rs`
3. Replace the placeholders with actual problem details
4. Add the module declaration to `src/problems/mod.rs`
5. Implement the solution and tests

### Template Usage

Replace the following placeholders in the template:
- `[Problem Number]` - LeetCode problem number
- `[Problem Title]` - Problem title
- `[Problem Description]` - Brief problem description
- `[module_name]` - Module name (e.g., `p594_longest_harmonious_subsequence`)
- `[method_name]` - Solution method name
- `[parameters]` - Method parameters
- `[return_type]` - Return type
- `[example_input]` - Example input for tests
- `[expected_output]` - Expected output for tests

## Features

- **Modular Architecture**: Each problem is in its own module with dedicated tests and benchmarks
- **Comprehensive Testing**: Unit tests, edge cases, and integration tests
- **Performance Benchmarking**: Built-in benchmarks for performance analysis
- **Template System**: Consistent structure for new problems
- **Multiple Solutions**: Different algorithmic approaches for comparison

## Problems Solved

- **594**: Longest Harmonious Subsequence
- **3330**: Find the Original Typed String I

## Performance Notes

The project includes benchmarks comparing different algorithmic approaches:
- HashMap-based solutions
- Sorting-based solutions  
- Optimized implementations using FxHashMap

Use `cargo bench` to see performance comparisons across different input sizes.