// Recursively combine adjacent pairs (preserves order, covers all parenthesizations)
// applying +,-,*,/ until one value remains; check |v-24|<1e-6.
// Time O(exponential in n), Space O(n) recursion. Here n=4.
function solve(nums) {
    if (nums.length === 1) return Math.abs(nums[0] - 24.0) < 1e-6;
    for (let i = 0; i + 1 < nums.length; i++) {
        const a = nums[i], b = nums[i + 1];
        const results = [a + b, a - b, a * b];
        if (Math.abs(b) > 1e-9) results.push(a / b);
        for (const r of results) {
            const next = nums.slice(0, i).concat([r], nums.slice(i + 2));
            if (solve(next)) return true;
        }
    }
    return false;
}

function canGet24(nums) {
    return solve(nums.map(Number));
}

console.log(canGet24([5, 2, 7, 8]) ? "True" : "False");
