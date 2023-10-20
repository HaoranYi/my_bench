fn main() {
    println!("Hello, world!");

    // Hash an input all at once.
    let hash1 = blake3::hash(b"foobarbaz");

    // Hash an input incrementally.
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"foo");
    hasher.update(b"bar");
    hasher.update(b"baz");
    let hash2 = hasher.finalize();
    assert_eq!(hash1, hash2);

    // Extended output. OutputReader also implements Read and Seek.
    let mut output = [0; 1000];
    let mut output_reader = hasher.finalize_xof();
    output_reader.fill(&mut output);
    assert_eq!(hash1, output[..32]);

    // Print a hash as hex.
    println!("{}", hash1);

    let bytes: Vec<u8> = (0..=255_u8).collect();

    let mut hasher = blake3::Hasher::new();
    hasher.update(&bytes[0..24]);

    hasher.update(&bytes[24..89]);
    let h1 = hasher.finalize();

    let mut hasher = blake3::Hasher::new();
    hasher.update(&bytes[0..8]);
    hasher.update(&bytes[8..16]);
    hasher.update(&bytes[16..24]);
    hasher.update(&bytes[24..25]);
    hasher.update(&bytes[25..57]);
    hasher.update(&bytes[57..89]);
    let h2 = hasher.finalize();

    // Print a hash as hex.
    println!("{}", h1);
    println!("{}", h2);

    assert_eq!(h1, h2);
}
