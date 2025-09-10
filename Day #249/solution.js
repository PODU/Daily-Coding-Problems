// Maximum XOR of any two elements using a binary trie of bits.
// Insert each number, greedily pick opposite bit per number. O(n*bits) time, O(n*bits) space.

const BITS = 32;

function findMaxXOR(nums) {
    const root = {};
    for (const x of nums) {
        let node = root;
        for (let i = BITS - 1; i >= 0; i--) {
            const b = (x >> i) & 1;
            if (!node[b]) node[b] = {};
            node = node[b];
        }
    }
    let best = 0;
    for (const x of nums) {
        let node = root;
        let cur = 0;
        for (let i = BITS - 1; i >= 0; i--) {
            const b = (x >> i) & 1;
            const want = b ^ 1;
            if (node[want]) {
                cur |= (1 << i);
                node = node[want];
            } else {
                node = node[b];
            }
        }
        if (cur > best) best = cur;
    }
    return best;
}

function main() {
    const nums = [3, 10, 5, 25, 2, 8];
    console.log(findMaxXOR(nums));
}

main();
