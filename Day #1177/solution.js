// Day 1177: Find the element appearing once while all others appear 3 times.
// Track bits seen once (ones) and twice (twos); a third sighting clears both.
// Time O(N), Space O(1).

function singleNumber(a) {
    let ones = 0, twos = 0;
    for (const x of a) {
        ones = (ones ^ x) & ~twos;
        twos = (twos ^ x) & ~ones;
    }
    return ones | 0; // 32-bit signed
}

console.log(singleNumber([6, 1, 3, 3, 3, 6, 6])); // 1
console.log(singleNumber([13, 19, 13, 13]));       // 19
