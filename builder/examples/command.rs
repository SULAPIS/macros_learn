use builder::Builder;

#[allow(dead_code)]
#[derive(Debug, Builder)]
struct Command {
    executable: String,
    #[builder(each = "arg", default = "Default::default()")]
    args: Vec<String>,
    #[builder(each = "env", default = "vec![\"RUST_LOG=info\".into()]")]
    env: Vec<String>,
    current_dir: Option<String>,
}
fn main() {
    let command = Command::builder()
        .executable("find")
        .arg("-c")
        .arg("-vvv")
        // .env("RUST_LOG=info")
        // .args(vec!["-c".into(), "-vvv".into()])
        // .env(vec![])
        .current_dir("D:\\Practice\\Rust\\macros_learn\\macros_learn")
        .finish()
        .unwrap();
    println!("{:#?}", command);
}
