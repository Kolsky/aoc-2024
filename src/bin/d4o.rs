pub fn run(input: &[u8]) -> u32 {
    let input = unsafe { input.align_to::<[u8; 141]>() }.1;
    assert_eq!(input.len(), 140);
    let mut sum = 0;
    for lanes in input.windows(3) {
        let (t, c, b) = (lanes[0], lanes[1], lanes[2]);
        for i in 1..139 {
            let (w, x, y, z) = (t[i - 1], t[i + 1], b[i - 1], b[i + 1]);
            if c[i] == b'A' && (w ^ x ^ y ^ z) == 0 && (x ^ y) != 0 {
                sum += u32::from((x ^ y) == (b'M' ^ b'S'));
            }
        }
    }
    sum
}

fn main() {
    let input = aoc_2024::input!(4).as_bytes();
    let sum = run(input);
    println!("{sum}");
}
