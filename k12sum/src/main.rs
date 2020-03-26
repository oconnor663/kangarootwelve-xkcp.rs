use anyhow::{bail, Context, Result};
use clap::{App, Arg};
use kangarootwelve_xkcp::Hasher;
use std::fs::File;
use std::io;
use std::io::prelude::*;

const FILE_ARG: &str = "file";
const LENGTH_ARG: &str = "length";
const NO_NAMES_ARG: &str = "no-names";
const RAW_ARG: &str = "raw";

fn clap_parse_argv() -> clap::ArgMatches<'static> {
    App::new("k12sum")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name(FILE_ARG).multiple(true))
        .arg(
            Arg::with_name(LENGTH_ARG)
                .long(LENGTH_ARG)
                .short("l")
                .takes_value(true)
                .value_name("LEN")
                .help(
                    "The number of output bytes, prior to hex\n\
                     encoding (default 32)",
                ),
        )
        .arg(
            Arg::with_name(NO_NAMES_ARG)
                .long(NO_NAMES_ARG)
                .help("Omits filenames in the output"),
        )
        .arg(Arg::with_name(RAW_ARG).long(RAW_ARG).help(
            "Writes raw output bytes to stdout, rather than hex.\n\
             --no-names is implied. In this case, only a single\n\
             input is allowed.",
        ))
        .get_matches()
}

// 64 KiB is the minimum needed to use AVX-512.
fn copy_wide(mut reader: impl Read, hasher: &mut Hasher) -> io::Result<u64> {
    let mut buffer = [0; 65536];
    let mut total = 0;
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => return Ok(total),
            Ok(n) => {
                hasher.update(&buffer[..n]);
                total += n as u64;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
}

fn hash_reader(reader: impl Read, output_len: usize) -> Result<Vec<u8>> {
    let mut hasher = Hasher::new();
    copy_wide(reader, &mut hasher)?;
    // TODO: a streaming reader
    let mut hash = vec![0; output_len];
    hasher.finalize(&mut hash);
    Ok(hash)
}

fn write_hex_output(output: &[u8]) -> Result<()> {
    print!("{}", hex::encode(output));
    Ok(())
}

fn write_raw_output(output: &[u8]) -> Result<()> {
    std::io::stdout().write_all(output)?;
    Ok(())
}

// Errors from this function get handled by the file loop and printed per-file.
fn hash_file(filepath: &std::ffi::OsStr, output_len: usize) -> Result<Vec<u8>> {
    let file = File::open(filepath)?;
    hash_reader(file, output_len)
}

fn main() -> Result<()> {
    let args = clap_parse_argv();
    let output_len = if let Some(length) = args.value_of(LENGTH_ARG) {
        length.parse::<usize>().context("Failed to parse length.")?
    } else {
        32
    };
    let print_names = !args.is_present(NO_NAMES_ARG);
    let raw_output = args.is_present(RAW_ARG);

    let mut did_error = false;
    if let Some(files) = args.values_of_os(FILE_ARG) {
        if raw_output && files.len() > 1 {
            bail!("k12sum: Only one filename can be provided when using --raw");
        }
        for filepath in files {
            let filepath_str = filepath.to_string_lossy();
            match hash_file(filepath, output_len) {
                Ok(output) => {
                    if raw_output {
                        write_raw_output(&output)?;
                    } else {
                        write_hex_output(&output)?;
                        if print_names {
                            println!("  {}", filepath_str);
                        } else {
                            println!();
                        }
                    }
                }
                Err(e) => {
                    did_error = true;
                    eprintln!("k12sum: {}: {}", filepath_str, e);
                }
            }
        }
    } else {
        let stdin = std::io::stdin();
        let stdin = stdin.lock();
        let output = hash_reader(stdin, output_len)?;
        if raw_output {
            write_raw_output(&output)?;
        } else {
            write_hex_output(&output)?;
            println!();
        }
    }
    std::process::exit(if did_error { 1 } else { 0 });
}