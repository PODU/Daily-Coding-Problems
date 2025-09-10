// Max of two ints without if/else/branch/ternary/comparison.
// Use sign bit of (a-b) via BigInt 64-bit diff to avoid overflow. O(1) time, O(1) space.

function maxOf(a, b) {
    const x = BigInt(a), y = BigInt(b);
    const d = x - y;
    const sign = (d >> 63n) & 1n; // 1 if a<b, else 0
    return x - sign * d;
}

function main() {
    console.log("max(3, 7) = " + maxOf(3, 7).toString());
    console.log("max(10, 2) = " + maxOf(10, 2).toString());
}

main();
