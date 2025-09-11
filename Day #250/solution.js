// Cryptarithmetic solver: backtracking over distinct letters with leading-zero pruning.
// Time: O(10!/(10-k)!) over k<=10 distinct letters; Space: O(k).

function solve(w1, w2, w3) {
    const order = [], seen = new Set();
    for (const w of [w1, w2, w3])
        for (const c of w)
            if (!seen.has(c)) { seen.add(c); order.push(c); }
    const leading = new Set([w1[0], w2[0], w3[0]]);
    const k = order.length;
    const assign = {};
    const used = new Array(10).fill(false);

    const num = (w) => { let n = 0; for (const c of w) n = n * 10 + assign[c]; return n; };

    function dfs(idx) {
        if (idx === k) return num(w1) + num(w2) === num(w3);
        for (let d = 0; d < 10; d++) {
            if (used[d]) continue;
            if (d === 0 && leading.has(order[idx])) continue;
            used[d] = true; assign[order[idx]] = d;
            if (dfs(idx + 1)) return true;
            used[d] = false; delete assign[order[idx]];
        }
        return false;
    }

    dfs(0);
    return order.map(c => [c, assign[c]]);
}

const result = solve("SEND", "MORE", "MONEY");
console.log("{" + result.map(([k, v]) => `'${k}': ${v}`).join(", ") + "}");
