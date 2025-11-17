// Test bit reversal implementation

fn bit_reverse(index: u32, log2_size: u32) -> u32 {
    let mut result = 0u32;
    let mut temp = index;

    for _ in 0..log2_size {
        result = (result << 1) | (temp & 1);
        temp >>= 1;
    }

    result
}

fn main() {
    // Test for size 8 (log2 = 3)
    println!("Bit reversal for size 8:");
    for i in 0..8 {
        let reversed = bit_reverse(i, 3);
        println!("  {} -> {}", i, reversed);
    }
    println!();

    // Expected for size 8:
    // 0 (000) -> 0 (000)
    // 1 (001) -> 4 (100)
    // 2 (010) -> 2 (010)
    // 3 (011) -> 6 (110)
    // 4 (100) -> 1 (001)
    // 5 (101) -> 5 (101)
    // 6 (110) -> 3 (011)
    // 7 (111) -> 7 (111)

    println!("Bit reversal for size 256 (first 16):");
    for i in 0..16 {
        let reversed = bit_reverse(i, 8);
        println!("  {} -> {}", i, reversed);
    }
}
