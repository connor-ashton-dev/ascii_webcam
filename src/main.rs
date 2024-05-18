use std::process::Command;
use std::thread;
use std::time::Duration;

mod pixels;
use pixels::generate_image;

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        // This uses ANSI escape codes to clear the screen and reset cursor position.
        print!("\x1B[2J\x1B[1;1H");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let image_capture_interval = Duration::from_millis(500);
    let image_file_path = "snapshot.jpg";

    // Image capturing thread
    let capture_thread = thread::spawn(move || loop {
        let _ = Command::new("imagesnap")
            .args(&["-w", "0.5", image_file_path])
            .output()
            .expect("Failed to capture image");
        // thread::sleep(image_capture_interval);
    });

    // Image processing thread
    let display_thread = thread::spawn(move || {
        loop {
            clear_screen();
            if let Err(e) = generate_image() {
                eprintln!("Error generating image: {}", e);
            }

            // Sleep a bit before trying to generate the next image
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Optional: Wait for threads to complete, or handle differently based on your app's needs
    capture_thread.join().unwrap();
    display_thread.join().unwrap();

    Ok(())
}
