use std::borrow::Cow;
use std::io::Write;
use std::process::{Command, Stdio};

const MAIN_PKG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../example_msgs");

/// This main function is used to generate the contents of lib.rs
/// Invoke the following from the workspace root to regen: `cargo run --bin roslibrust_test > ./roslibrust_test/src/lib.rs`
fn main() {
    let tokens = roslibrust::find_and_generate_ros_messages(vec![MAIN_PKG_PATH.into()]);
    let source = format_rust_source(tokens.to_string().as_str()).to_string();
    println!("{}", source);
}

fn format_rust_source(source: &str) -> Cow<'_, str> {
    if let Ok(mut process) = Command::new("rustfmt")
        .arg("--emit=stdout")
        .arg("--edition=2021")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        {
            let stdin = process.stdin.as_mut().unwrap();
            stdin.write_all(source.as_bytes()).unwrap()
        }
        if let Ok(output) = process.wait_with_output() {
            if output.status.success() {
                return std::str::from_utf8(&output.stdout[..])
                    .unwrap()
                    .to_owned()
                    .into();
            }
        }
    }
    Cow::Borrowed(source)
}

#[cfg(test)]
mod test {
    use crate::*;

    /// Confirms that codegen has been run and changes committed
    #[test]
    fn lib_is_up_to_date() {
        let tokens = roslibrust::find_and_generate_ros_messages(vec![MAIN_PKG_PATH.into()]);
        let source = format_rust_source(tokens.to_string().as_str()).to_string();
        let lib_path = env!("CARGO_MANIFEST_DIR").to_string() + "/src/lib.rs";
        let lib_contents =
            std::fs::read_to_string(lib_path).expect("Failed to load current lib.rs contents");

        // Creating a diff so if there are changes output in CI is sane
        let diff = diffy::create_patch(&source, &lib_contents);
        println!("Diff is \n{}", diff);

        if source.trim() != lib_contents.trim() {
            panic!("Changes detected see diff!");
        }
    }
}
