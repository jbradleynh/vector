use ansi_term::Colour;
use remap::{prelude::*, Program, Runtime};
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut failed_count = 0;
    let tests = fs::read_dir("tests").expect("dir exists");

    for file in tests {
        let mut test = Test::new(file.expect("file").path());

        print!("{:.<30}", test.name);

        if test.skip {
            println!("{}", Colour::Yellow.bold().paint("SKIPPED"));
        }

        let state = state::Program::default();
        let mut runtime = Runtime::new(state);
        let program = Program::new(&test.source, &remap_functions::all(), None, true);
        let want = test.result.to_string();

        match program {
            Ok(program) => {
                let result = runtime.execute(&mut test.object, &program);

                match result {
                    Ok(value) => {
                        let got = value.to_string();

                        if got == want {
                            println!("{}", Colour::Green.bold().paint("OK"));
                        } else {
                            println!("{} (expectation)", Colour::Red.bold().paint("FAILED"));

                            let diff =
                                prettydiff::diff_chars(&got, &want).set_highlight_whitespace(true);
                            println!("  {}", diff);

                            failed_count += 1;
                        }
                    }
                    Err(err) => {
                        let got = err.to_string();
                        if got == want {
                            println!("{}", Colour::Green.bold().paint("OK"));
                        } else {
                            println!("{} (runtime)", Colour::Red.bold().paint("FAILED"));
                            println!("{}", err);
                        }
                    }
                }
            }
            Err(err) => {
                let got = err.to_string().trim().to_owned();
                let want = want.trim().to_owned();
                if got == want {
                    println!("{}", Colour::Green.bold().paint("OK"));
                } else {
                    println!("{} (compilation)", Colour::Red.bold().paint("FAILED"));

                    let diff = prettydiff::diff_chars(&got, &want).set_highlight_whitespace(true);
                    println!("{}", diff);
                }
            }
        }
    }

    if failed_count > 0 {
        std::process::exit(1)
    }
}

#[derive(Debug)]
struct Test {
    name: String,
    source: String,
    object: Value,
    result: String,
    skip: bool,
}

enum CaptureMode {
    Result,
    Object,
    None,
}

impl Test {
    fn new(path: PathBuf) -> Self {
        let name = path
            .to_string_lossy()
            .strip_prefix("tests/")
            .expect("test")
            .to_owned();

        let content = fs::read_to_string(path).expect("content");

        let mut source = String::new();
        let mut object = String::new();
        let mut result = String::new();
        let mut skip = false;

        if content.starts_with("# SKIP") {
            skip = true;
        }

        let mut capture_mode = CaptureMode::None;
        for mut line in content.lines() {
            if line.starts_with('#') {
                line = line.strip_prefix('#').expect("prefix");
                line = line.strip_prefix(' ').unwrap_or(line);

                if line.starts_with("object:") {
                    capture_mode = CaptureMode::Object;
                    line = line.strip_prefix("object:").expect("object").trim_start();
                } else if line.starts_with("result:") {
                    capture_mode = CaptureMode::Result;
                    line = line.strip_prefix("result:").expect("result").trim_start();
                }

                match capture_mode {
                    CaptureMode::None => continue,
                    CaptureMode::Result => {
                        result.push_str(line);
                        result.push('\n');
                    }
                    CaptureMode::Object => {
                        object.push_str(line);
                    }
                }
            } else {
                source.push_str(line);
                source.push('\n')
            }
        }

        let object = serde_json::from_str::<'_, Value>(&object).expect("valid object");

        result = result.trim_end().to_owned();

        Self {
            name,
            source,
            object,
            result,
            skip,
        }
    }
}
