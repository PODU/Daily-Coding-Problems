// Maximum XOR of two elements using a binary trie (bits high->low), greedy opposite bit.
// Time O(n*B), Space O(n*B), B = 31.
'use strict';

const B = 31;

function maximumXOR(nums) {
    const root = {};
    let best = 0;
    for (const x of nums) {
        let ins = root, cur = root, curXor = 0;
        for (let b = B - 1; b >= 0; b--) {
            const bit = (x >> b) & 1;
            if (!ins[bit]) ins[bit] = {};
            ins = ins[bit];
            const want = bit ^ 1;
            if (cur[want]) { curXor |= (1 << b); cur = cur[want]; }
            else if (cur[bit]) cur = cur[bit];
        }
        best = Math.max(best, curXor);
    }
    return best;
}

const nums = [3, 10, 5, 25, 2, 8];
console.log(maximumXOR(nums)); // 28
