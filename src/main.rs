use scap::capturer::{self, Capturer, Options};

fn main() {
    if !scap::is_supported() {
        println!("Scap is not supported");
    }

    if !scap::has_permission() {
        println!("Permission not granted");
        println!("Requesting permission...");

        if !scap::request_permission() {
            println!("Permission denied");
            return;
        }
    }
    let options = Options {
        fps: 60,
        target: None,
        show_cursor: true,
        ..Default::default()
    };

    let mut capturer = Capturer::new(options);
    let mut input = String::new();
    println!("Capturing... Press enter to stop");
    std::io::stdin().read_line(&mut input).unwrap();

    capturer.stop_capture();
    println!("Capture stopped");
}
