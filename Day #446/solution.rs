// Day 446: Power of four test in O(1) (better than O(log N)).
// Power of two (single set bit) AND that bit sits at an even position.

fn is_power_of_four(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0 && (n & 0x5555_5555) != 0
}

fn main() {
    for n in [1u32, 4, 8, 16, 64, 0, 5, 256] {
        println!("{} -> {}", n, is_power_of_four(n));
    }
    // 1->true 4->true 8->false 16->true 64->true 0->false 5->false 256->true
}
