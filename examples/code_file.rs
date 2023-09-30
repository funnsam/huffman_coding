use simple_huffman::*;

fn main() {
    let mut args = std::env::args();
    let mode = match args.nth(1).expect("usage: ... [e/d] [main file] [tree file] [output]").as_str() {
        "e" => true, "d" => false,
        _ => panic!("usage: ... [e/d] [main file] [tree file] [output]")
    };
    let main = args.next().expect("usage: ... [e/d] [main file] [tree file] [output]");
    let tree = args.next().expect("usage: ... [e/d] [main file] [tree file] [output]");
    let out  = args.next().expect("usage: ... [e/d] [main file] [tree file] [output]");

    if mode { // encode
        let main = std::fs::read(main).expect("main file not found");
        let m = main.len();
        let (o, t) = huffman_encode(main);
        let t = postcard::to_stdvec(&t).unwrap();
        println!("\x1b[1;32mDone:\x1b[0;1m compression rate:\x1b[0m {:.02}%", (m as f32 / (o.len() + t.len()) as f32) * 100.0);
        std::fs::write(out, o).unwrap();
        std::fs::write(tree, t).unwrap();
    } else {  // decode
        let main = std::fs::read(main).expect("main file not found");
        let tree = std::fs::read(tree).expect("tree file not found");
        let tree = postcard::from_bytes(&tree).unwrap();
        let decoded = huffman_decode(main, tree);
        std::fs::write(out, decoded).unwrap();
        println!("\x1b[1;32mDone\x1b[0m");
    }
}
