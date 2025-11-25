// Power set via bitmasks; sort subsets by size then numeric order. Time O(n*2^n), Space O(2^n).
function powerSet(s) {
    const n = s.length;
    const subsets = [];
    for (let mask = 0; mask < (1 << n); mask++) {
        const sub = [];
        for (let i = 0; i < n; i++)
            if (mask & (1 << i)) sub.push(s[i]);
        subsets.push(sub);
    }
    subsets.sort((a, b) => {
        if (a.length !== b.length) return a.length - b.length;
        for (let i = 0; i < a.length; i++)
            if (a[i] !== b[i]) return a[i] - b[i];
        return 0;
    });
    return subsets;
}

const s = [1, 2, 3];
const subsets = powerSet(s);
const parts = subsets.map(sub => "{" + sub.join(", ") + "}");
console.log("{" + parts.join(", ") + "}");
