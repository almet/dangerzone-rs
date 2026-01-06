use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

/// A simple Dangerzone CLI implementation in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input document path
    #[arg(short, long)]
    input: String,

    /// Output PDF path
    #[arg(short, long)]
    output: String,

    /// Use docker instead of podman
    #[arg(long, default_value = "false")]
    use_docker: bool,
}

const IMAGE_NAME: &str = "ghcr.io/freedomofpress/dangerzone/v1";

fn get_runtime_name(use_docker: bool) -> &'static str {
    if use_docker {
        "docker"
    } else {
        "podman"
    }
}

fn get_security_args() -> Vec<String> {
    vec![
        "--log-driver".to_string(),
        "none".to_string(),
        "--security-opt".to_string(),
        "no-new-privileges".to_string(),
        "--cap-drop".to_string(),
        "all".to_string(),
        "--cap-add".to_string(),
        "SYS_CHROOT".to_string(),
        "--security-opt".to_string(),
        "label=type:container_engine_t".to_string(),
        "--network=none".to_string(),
        "-u".to_string(),
        "dangerzone".to_string(),
    ]
}

fn convert_doc_to_pixels(
    runtime: &str,
    input_path: &str,
) -> Result<Vec<u8>> {
    eprintln!("Converting document to pixels...");

    let mut args = vec!["run".to_string()];
    args.extend(get_security_args());
    args.extend(vec![
        "--rm".to_string(),
        "-i".to_string(),
        IMAGE_NAME.to_string(),
        "/usr/bin/python3".to_string(),
        "-m".to_string(),
        "dangerzone.conversion.doc_to_pixels".to_string(),
    ]);

    let mut child = Command::new(runtime)
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to spawn container")?;

    // Read the input document
    let mut input_file = File::open(input_path)
        .context("Failed to open input file")?;
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)
        .context("Failed to read input file")?;

    // Write the document to the container's stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(&input_data)
            .context("Failed to write to container stdin")?;
    }

    // Read the output from the container
    let output = child.wait_with_output()
        .context("Failed to wait for container")?;

    if !output.status.success() {
        anyhow::bail!("Container failed with status: {}", output.status);
    }

    eprintln!("Document converted to pixels successfully");
    Ok(output.stdout)
}

fn convert_pixels_to_pdf(
    runtime: &str,
    pixels_data: &[u8],
    output_path: &str,
) -> Result<()> {
    eprintln!("Converting pixels to safe PDF...");

    let mut args = vec!["run".to_string()];
    args.extend(get_security_args());
    args.extend(vec![
        "--rm".to_string(),
        "-i".to_string(),
        IMAGE_NAME.to_string(),
        "/usr/bin/python3".to_string(),
        "-m".to_string(),
        "dangerzone.conversion.pixels_to_pdf".to_string(),
    ]);

    let mut child = Command::new(runtime)
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .context("Failed to spawn container")?;

    // Write the pixels data to the container's stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(pixels_data)
            .context("Failed to write pixels to container stdin")?;
    }

    // Read the output PDF from the container
    let output = child.wait_with_output()
        .context("Failed to wait for container")?;

    if !output.status.success() {
        anyhow::bail!("Container failed with status: {}", output.status);
    }

    // Write the safe PDF to the output file
    let mut output_file = File::create(output_path)
        .context("Failed to create output file")?;
    output_file.write_all(&output.stdout)
        .context("Failed to write output file")?;

    eprintln!("Safe PDF created successfully at: {}", output_path);
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let runtime = get_runtime_name(args.use_docker);

    eprintln!("Dangerzone Rust CLI");
    eprintln!("Using container runtime: {}", runtime);
    eprintln!("Input: {}", args.input);
    eprintln!("Output: {}", args.output);

    // Step 1: Convert document to pixels
    let pixels_data = convert_doc_to_pixels(runtime, &args.input)?;

    // Step 2: Convert pixels to safe PDF
    convert_pixels_to_pdf(runtime, &pixels_data, &args.output)?;

    eprintln!("Conversion completed successfully!");
    Ok(())
}
