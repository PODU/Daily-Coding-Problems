// Power set via bitmask iteration over 2^n subsets, then sorted by (size, elements).
// Time: O(n*2^n), Space: O(n*2^n) to hold all subsets.

function powerSet(s) {
    const n = s.length;
    const subsets = [];
    for (let mask = 0; mask < (1 << n); mask++) {
        const cur = [];
        for (let i = 0; i < n; i++)
            if (mask & (1 << i)) cur.push(s[i]);
        subsets.push(cur);
    }
    subsets.sort((a, b) => {
        if (a.length !== b.length) return a.length - b.length;
        for (let i = 0; i < a.length; i++)
            if (a[i] !== b[i]) return a[i] - b[i];
        return 0;
    });
    return subsets;
}

function main() {
    const s = [1, 2, 3];
    const subsets = powerSet(s);
    const parts = subsets.map(sub => "{" + sub.join(", ") + "}");
    console.log("{" + parts.join(", ") + "}");
}

main();
