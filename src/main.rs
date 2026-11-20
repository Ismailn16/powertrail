use std::process::Command;
fn main() {
    let output = Command::new("ioreg")
        .args(["-w0", "-rn", "AppleSmartBattery"])
        .output()
        .expect("Failed to run ioreg");

    let text = String::from_utf8_lossy(&output.stdout);

    println!("{text}");
}
