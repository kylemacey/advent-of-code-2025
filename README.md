# Advent of Code 2025 🎄

My solutions for [Advent of Code 2025](https://adventofcode.com/2025) written in Rust.

> **Note:** The scaffolding tool and this README were generated with assistance from an LLM to automate setup. All Advent of Code puzzle solutions are my own work.

## Quick Start

1. **Get your AOC session cookie:**
   - Go to [adventofcode.com](https://adventofcode.com/2025) and log in
   - Open browser DevTools (F12)
   - Go to Application/Storage → Cookies → https://adventofcode.com
   - Copy the **value** of the `session` cookie

2. **Set environment variable:**
   ```bash
   export AOC_SESSION=your_session_cookie_here

   # Optional: Make it permanent
   echo 'export AOC_SESSION=your_session_cookie' >> ~/.bashrc
   ```

3. **Scaffold and solve:**
   ```bash
   ./aoc new          # Scaffold today's challenge
   # Edit dayXX/src/main.rs with your solution
   ./aoc test 1       # Run tests
   ./aoc run 1        # Run solution
   ```

That's it! The scaffold tool automatically:
- Creates the day's directory structure
- Downloads the problem description to `problem.md`
- Downloads your puzzle input to `input.txt`
- Generates a code template with `part1()` and `part2()` functions
- Updates the workspace `Cargo.toml`

Run `./aoc new` again after completing Part 1 to fetch Part 2!

## Helper Commands

### Using the `aoc` script (recommended):
```bash
./aoc new [day]       # Scaffold a new day (defaults to today in December)
./aoc run [day]       # Run a day's solution
./aoc test [day]      # Run tests
./aoc release [day]   # Run with optimizations
```

### Using Make:
```bash
make new DAY=5        # Scaffold day 5
make run DAY=3        # Run day 3
make test DAY=7       # Test day 7
make help             # Show all commands
```

### Using Cargo directly:
```bash
cargo run --bin day01                  # Run day 1
cargo test --bin day01                 # Run tests
cargo run --release --bin day01        # Run optimized
```

## Example Daily Workflow

```bash
# Morning: New puzzle releases (midnight EST)
./aoc new                   # Creates day directory, downloads problem & input

# Solve Part 1
cat day01/problem.md        # Read the problem
vim day01/src/main.rs       # Implement solution
./aoc test 1                # Test with examples
./aoc run 1                 # Run on real input
# Submit Part 1 answer on website

# Fetch Part 2
./aoc new                   # Same command! Detects day exists
vim day01/src/main.rs       # Implement Part 2
./aoc run 1                 # Run both parts
# Submit Part 2 answer
# ⭐⭐ Both stars earned!

# Next day
./aoc new                   # Auto-creates day02
```

## Project Structure

This is a Cargo workspace with each day as a separate binary crate:

```
.
├── aoc                    # Helper script
├── Makefile              # Make commands
├── Cargo.toml            # Workspace config
├── scaffold/             # Scaffolding tool source
│   └── src/main.rs
├── day01/
│   ├── Cargo.toml
│   ├── input.txt         # Your puzzle input (gitignored)
│   ├── problem.md        # Problem description (gitignored)
│   └── src/main.rs       # Your solution
├── day02/
│   └── ...
└── target/               # Build artifacts
    ├── debug/dayXX       # Debug builds
    └── release/dayXX     # Release builds
```

**Note:** Binaries are output to `target/debug/` or `target/release/` - this is the standard Rust convention (not `bin/` or `exe/`).

## Code Template

Each generated `main.rs` includes:

```rust
use std::fs;

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

fn part1(data: &[String]) -> usize {
    // TODO: Implement part 1
    0
}

fn part2(data: &[String]) -> usize {
    // TODO: Implement part 2
    0
}

fn main() {
    let input = fs::read_to_string("dayXX/input.txt")
        .expect("Failed to read input file");
    let data = parse_input(&input);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
TODO: Add example input here
";

    #[test]
    fn test_part1() {
        let data = parse_input(EXAMPLE);
        assert_eq!(part1(&data), 0);
    }
}
```

Customize `parse_input()` for your data structure, then implement `part1()` and `part2()`.

## Available Dependencies

Common AOC dependencies are pre-configured in the workspace:

- **`itertools`** - Extended iterator functionality
- **`regex`** - Regular expressions
- **`nom`** - Parser combinators
- **`rayon`** - Data parallelism

Add to a day's `Cargo.toml`:
```toml
[dependencies]
itertools.workspace = true
regex.workspace = true
```

## Setup

### DevContainer (Recommended)
This project includes a devcontainer. Open in VS Code with the Dev Containers extension, and it will automatically set up the Rust toolchain.

### Manual Setup
Ensure Rust is installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Troubleshooting

**"AOC_SESSION environment variable not set"**
- Export your session cookie: `export AOC_SESSION=your_cookie`

**"Failed to fetch problem: HTTP 404"**
- The day hasn't been released yet (midnight EST)
- Check your day number

**"Failed to fetch problem: HTTP 400/401"**
- Your session cookie expired - get a fresh one from your browser

**"Not December, please specify a day number"**
- Outside December, explicitly specify: `./aoc new 1`

**Binary architecture error**
- The project uses `cargo run`, which builds for your architecture automatically

**Want to start fresh on a day?**
```bash
rm -rf day05
./aoc new 5
```

## Git & Sharing

The `.gitignore` is configured to:
- ✅ Include: Source code, scaffold tool, helper scripts
- ❌ Exclude: `input.txt` and `problem.md` files (per AOC's sharing policy)
- ❌ Exclude: `target/` build artifacts

Your solutions are yours to share, but respect [Advent of Code's guidelines](https://adventofcode.com/2025/about) about not sharing inputs or problem text.

## Tips

1. **Parse early, parse well** - A good `parse_input()` makes part1/part2 much easier
2. **Use the test module** - Add examples from the problem description
3. **Read files once** - The template already loads `input.txt` for you
4. **Running `./aoc new` is safe** - It won't overwrite your existing solutions
5. **Release mode for slow solutions** - Use `./aoc release 15` if runtime is high

## License

Solutions are my own. The scaffolding tool and project structure are freely available for anyone doing Advent of Code.

---

Happy coding! 🎄🦀⭐
