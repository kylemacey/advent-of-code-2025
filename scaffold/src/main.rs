// This scaffold was generated with LLM.

use anyhow::{anyhow, Context, Result};
use chrono::{Datelike, Local};
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const YEAR: i32 = 2025;
const AOC_URL: &str = "https://adventofcode.com";

fn get_session_cookie() -> Result<String> {
    env::var("AOC_SESSION").context(
        "AOC_SESSION environment variable not set. \
         Get your session cookie from adventofcode.com and set it with:\n\
         export AOC_SESSION=your_session_cookie_here",
    )
}

fn get_day_from_args_or_current() -> Result<u32> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        args[1]
            .parse::<u32>()
            .context("Invalid day number provided")
    } else {
        // Get current day if in December
        let now = Local::now();
        if now.month() == 12 {
            Ok(now.day())
        } else {
            Err(anyhow!("Not December, please specify a day number"))
        }
    }
}

fn fetch_problem_text(client: &Client, day: u32, part: u32) -> Result<String> {
    let url = format!("{AOC_URL}/{YEAR}/day/{day}");
    let response = client
        .get(&url)
        .send()
        .context("Failed to fetch problem page")?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to fetch problem: HTTP {}",
            response.status()
        ));
    }

    let html = response.text()?;
    let document = Html::parse_document(&html);

    // Select all article tags (AOC uses <article> for problem descriptions)
    let article_selector = Selector::parse("article.day-desc").unwrap();
    let articles: Vec<_> = document.select(&article_selector).collect();

    if articles.is_empty() {
        return Err(anyhow!("No problem description found on the page"));
    }

    // Part 1 is the first article, Part 2 is the second (if it exists)
    let article = if part == 2 && articles.len() >= 2 {
        articles[1]
    } else if part == 1 {
        articles[0]
    } else {
        return Err(anyhow!("Part {part} not available yet"));
    };

    // Convert HTML to a more readable format
    let text = article.text().collect::<Vec<_>>().join("");

    // Add some formatting
    let formatted = format!("# Day {} - Part {}\n\n{}\n", day, part, text.trim());

    Ok(formatted)
}

fn fetch_input(client: &Client, day: u32) -> Result<String> {
    let url = format!("{AOC_URL}/{YEAR}/day/{day}/input");
    let response = client.get(&url).send().context("Failed to fetch input")?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to fetch input: HTTP {}", response.status()));
    }

    response.text().context("Failed to read input text")
}

fn create_day_structure(day: u32) -> Result<PathBuf> {
    let day_dir = PathBuf::from(format!("day{day:02}"));
    let src_dir = day_dir.join("src");

    fs::create_dir_all(&src_dir).context(format!(
        "Failed to create directory structure for day {day}"
    ))?;

    Ok(day_dir)
}

fn write_cargo_toml(day_dir: &Path, day: u32) -> Result<()> {
    let cargo_toml = format!(
        r#"[package]
name = "day{day:02}"
version.workspace = true
edition.workspace = true
authors.workspace = true

[dependencies]
itertools.workspace = true
"#
    );

    fs::write(day_dir.join("Cargo.toml"), cargo_toml).context("Failed to write Cargo.toml")?;

    Ok(())
}

fn write_main_rs(day_dir: &Path, day: u32) -> Result<()> {
    let main_rs = format!(
        r#"use std::fs;

fn parse_input(input: &str) -> Vec<String> {{
    input.lines().map(|s| s.to_string()).collect()
}}

fn part1(data: &[String]) -> usize {{
    // TODO: Implement part 1
    0
}}

fn part2(data: &[String]) -> usize {{
    // TODO: Implement part 2
    0
}}

fn main() {{
    let input = fs::read_to_string("day{day:02}/input.txt")
        .expect("Failed to read input file");

    let data = parse_input(&input);

    println!("Part 1: {{}}", part1(&data));
    println!("Part 2: {{}}", part2(&data));
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const EXAMPLE: &str = "\
TODO: Add example input here
";

    #[test]
    fn test_part1() {{
        let data = parse_input(EXAMPLE);
        assert_eq!(part1(&data), 0);
    }}

    #[test]
    fn test_part2() {{
        let data = parse_input(EXAMPLE);
        assert_eq!(part2(&data), 0);
    }}
}}
"#
    );

    fs::write(day_dir.join("src/main.rs"), main_rs).context("Failed to write main.rs")?;

    Ok(())
}

fn write_problem_md(day_dir: &Path, problem_text: &str, part: u32) -> Result<()> {
    let problem_file = if part == 1 {
        day_dir.join("problem.md")
    } else {
        day_dir.join("problem_part2.md")
    };

    // If part 2, append to existing file or create new
    if part == 2 && problem_file.exists() {
        let existing = fs::read_to_string(&problem_file)?;
        fs::write(
            &problem_file,
            format!("{existing}\n\n---\n\n{problem_text}"),
        )?;
    } else {
        fs::write(&problem_file, problem_text)
            .context(format!("Failed to write problem.md for part {part}"))?;
    }

    Ok(())
}

fn write_input(day_dir: &Path, input: &str) -> Result<()> {
    fs::write(day_dir.join("input.txt"), input).context("Failed to write input.txt")?;
    Ok(())
}

fn day_exists(day: u32) -> bool {
    PathBuf::from(format!("day{day:02}")).exists()
}

fn update_workspace_members(day: u32) -> Result<()> {
    let cargo_toml_path = PathBuf::from("Cargo.toml");
    let content =
        fs::read_to_string(&cargo_toml_path).context("Failed to read workspace Cargo.toml")?;

    let day_name = format!("day{day:02}");

    // Check if day is already in members
    if content.contains(&format!("\"{day_name}\"")) {
        return Ok(()); // Already added
    }

    // Find the members array and add the new day
    let lines: Vec<&str> = content.lines().collect();
    let mut new_content = String::new();
    let mut in_members = false;
    let mut added = false;

    for line in lines {
        if line.trim().starts_with("members = [") {
            in_members = true;
        }

        if in_members && line.contains(']') && !added {
            // Add new day before the closing bracket
            if line.trim() == "]" {
                // Empty members or just scaffold
                new_content.push_str(&format!("    \"{day_name}\",\n"));
            } else {
                // Has content before ]
                new_content.push_str(&format!("    \"{day_name}\",\n"));
            }
            added = true;
            in_members = false;
        }

        new_content.push_str(line);
        new_content.push('\n');
    }

    fs::write(&cargo_toml_path, new_content).context("Failed to write updated Cargo.toml")?;

    Ok(())
}

fn main() -> Result<()> {
    let day = get_day_from_args_or_current()?;
    let _session = get_session_cookie()?;

    println!("🎄 Scaffolding Advent of Code {YEAR} - Day {day}");

    // Create HTTP client with session cookie
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::COOKIE,
        format!("session={_session}").parse()?,
    );

    let client = Client::builder()
        .user_agent("github.com/yourusername/advent-of-code-2025 by your@email.com")
        .default_headers(headers)
        .build()?;

    let day_dir_exists = day_exists(day);

    // Determine which part we're fetching
    let part = if day_dir_exists {
        println!("📂 Day {day:02} already exists, fetching Part 2...");
        2
    } else {
        println!("📂 Creating new day {day:02} structure...");
        1
    };

    // Fetch problem description
    println!("📥 Fetching problem description (Part {part})...");
    let problem_text = fetch_problem_text(&client, day, part)?;

    // Fetch input
    println!("📥 Fetching input...");
    let input = fetch_input(&client, day)?;

    // Create or update day structure
    let day_dir = if !day_dir_exists {
        let dir = create_day_structure(day)?;
        write_cargo_toml(&dir, day)?;
        write_main_rs(&dir, day)?;

        // Update workspace Cargo.toml
        println!("📝 Updating workspace Cargo.toml...");
        update_workspace_members(day)?;

        dir
    } else {
        PathBuf::from(format!("day{day:02}"))
    };

    // Write problem and input
    write_problem_md(&day_dir, &problem_text, part)?;
    write_input(&day_dir, &input)?;

    println!("✅ Success!");
    println!("\n📖 Problem saved to: day{day:02}/problem.md");
    if part == 2 {
        println!("   (Part 2 appended to problem.md)");
    }
    println!("📄 Input saved to: day{day:02}/input.txt");
    println!("🦀 Code ready at: day{day:02}/src/main.rs");
    println!("\n🚀 Run with: cargo run --bin day{day:02}");

    Ok(())
}
