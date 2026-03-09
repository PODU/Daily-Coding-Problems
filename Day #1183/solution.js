// Day 1183: Generate all N-digit strobogrammatic numbers (same when rotated 180).
// Recursively build from outside in using rotation pairs; drop leading zeros.
// Time O(output size), Space O(N) recursion depth.

const PAIRS = [['0','0'],['1','1'],['6','9'],['8','8'],['9','6']];
const ROT = {'0':'0','1':'1','6':'9','8':'8','9':'6'};

function helper(m) {
    if (m === 0) return [""];
    if (m === 1) return ["0", "1", "8"];
    const inner = helper(m - 2), res = [];
    for (const s of inner)
        for (const [a, b] of PAIRS)
            res.push(a + s + b);
    return res;
}

function strobogrammatic(n) {
    return helper(n)
        .filter(s => !(s.length > 1 && s[0] === '0') && s !== "0")
        .map(Number)
        .sort((x, y) => x - y);
}

function isStrobo(s) {
    let l = 0, r = s.length - 1;
    while (l <= r) {
        if (ROT[s[l]] === undefined || ROT[s[l]] !== s[r]) return false;
        l++; r--;
    }
    return true;
}

console.log(`2-digit strobogrammatic numbers: [${strobogrammatic(2).join(", ")}]`);
console.log(`16891 is strobogrammatic: ${isStrobo("16891")}`);
