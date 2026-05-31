use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: regex-compiler <pattern> <text>");
        println!("Example: regex-compiler hello helloworld");
        return;
    }

    let pattern = &args[1];
    let text = &args[2];

    println!("[*] Checking pattern: '{}' in text: '{}'", pattern, text);
    if text.contains(pattern) {
        println!("[+] MATCH FOUND!");
    } else {
        println!("[-] NO MATCH.");
    }
}