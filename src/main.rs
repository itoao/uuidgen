use arboard::Clipboard;
use clap::Parser;
use uuid::Uuid;

#[derive(Parser)]
#[command(
    name = "uuidgen",
    about = "generate a universally unique identifier",
    override_usage = "uuidgen"
)]
struct Args {
    #[arg(default_value_t = 1)]
    count: u32,
}

fn main() {
    let args = Args::parse();
    let mut clipboard = Clipboard::new().unwrap();
    let uuids = generate_uuids(args.count);
    let result = join_uuids(&uuids);

    print_uuids(&uuids);
    copy_to_clipboard(&mut clipboard, &result);
    print_success_message();
}

fn generate_uuids(count: u32) -> Vec<String> {
    (0..count).map(|_| Uuid::new_v4().to_string()).collect()
}

fn join_uuids(uuids: &[String]) -> String {
    uuids.join("\n")
}

fn print_uuids(uuids: &[String]) {
    for uuid in uuids {
        println!("{uuid}");
    }
}

fn copy_to_clipboard(clipboard: &mut Clipboard, text: &str) {
    clipboard.set_text(text).unwrap();
}

fn print_success_message() {
    println!("\x1b[32mCopied to clipboard! 📋\x1b[0m");
}
