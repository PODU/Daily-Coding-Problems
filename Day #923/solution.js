// Power of four via O(1) bit manipulation.
// power of two: n>0 && (n&(n-1))==0; set bit in even position: (n & 0x55555555)!=0.
// Time O(1), Space O(1).
function isPowerOfFour(n) {
    return n > 0 && (n & (n - 1)) === 0 && (n & 0x55555555) !== 0;
}

function main() {
    for (const n of [1, 4, 16, 64, 8, 5, 0]) {
        console.log(`${n}: ${isPowerOfFour(n)}`);
    }
}

main();
