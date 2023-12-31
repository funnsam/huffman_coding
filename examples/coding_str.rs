use simple_huffman::*;

fn main() {
    let before = "According to all known laws of aviation, there is no way that a bee should be able to fly. Its wings are too small to get its fat little body off the ground. The bee, of course, flies anyways. Because bees don't care what humans think is impossible.";
    let (after, tree) = huffman_encode(before.chars().collect::<Vec<char>>());
    println!("\x1b[1mBefore:\x1b[0m {before:?} ({} bytes)", before.len());
    print!("\x1b[1mCompressed:\x1b[0m ");
    for i in after.iter() {
        print!("{i:02x}");
    }
    println!(" ({} bytes)", after.len());
    println!("\x1b[1mCompression rate:\x1b[0m {:.1}%", (before.len() as f32 / after.len() as f32) * 100.0);

    let decoded = huffman_decode(after, tree);
    println!("\x1b[1mDecompressed:\x1b[0m {:?}", String::from_iter(decoded));
}
