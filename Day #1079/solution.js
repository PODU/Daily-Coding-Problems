// XOR all -> a^b; isolate via lowest set bit; partition & XOR each group to recover a,b; O(n) time O(1) space

function findTwo(nums) {
    let xorAll = 0;
    for (const n of nums) xorAll ^= n;
    const bit = xorAll & (-xorAll);          // lowest set bit that differs between a and b
    let a = 0, b = 0;
    for (const n of nums) {
        if (n & bit) a ^= n;
        else         b ^= n;
    }
    return [Math.min(a, b), Math.max(a, b)];
}

const nums = [2, 4, 6, 8, 10, 2, 6, 10];
const [a, b] = findTwo(nums);
console.log(`${a} and ${b}`);
