use honggfuzz::fuzz;
use tere::settings::parse_keymap_arg;

fn main() {
    // Define the fuzzing loop
    loop {
        // Fuzzing input will be provided by Honggfuzz
        fuzz!(|data: &[u8]| {
            // Convert the input to a string
            if let Ok(name) = std::str::from_utf8(data) {
                // Call the parse_keymap_arg function with the fuzzed input
                let _ = parse_keymap_arg(name);

            }
        });
    }
}