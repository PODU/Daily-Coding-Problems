// Day 268: Power of four check in O(1).
// Power of two (n & (n-1))==0 AND single bit at even position (n & 0x55555555). Time O(1), Space O(1).

function isPowerOfFour(n) {
    return n !== 0 && (n & (n - 1)) === 0 && (n & 0x55555555) !== 0;
}

for (const t of [16, 8, 64, 5]) {
    console.log(`${t} -> ${isPowerOfFour(t) ? "True" : "False"}`);
}
