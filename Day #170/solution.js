// Word ladder via BFS over one-letter transformations. O(N * L * 26) time, O(N) space.

function ladder(start, end, dictionary) {
    if (start === end) return [start];
    const dict = new Set(dictionary);
    const queue = [start];
    const parent = new Map();
    parent.set(start, null);
    while (queue.length) {
        const cur = queue.shift();
        for (let i = 0; i < cur.length; i++) {
            for (let c = 97; c <= 122; c++) {
                const ch = String.fromCharCode(c);
                if (ch === cur[i]) continue;
                const nxt = cur.slice(0, i) + ch + cur.slice(i + 1);
                if (dict.has(nxt) && !parent.has(nxt)) {
                    parent.set(nxt, cur);
                    if (nxt === end) {
                        const path = [];
                        let s = end;
                        while (s !== null) { path.push(s); s = parent.get(s); }
                        return path.reverse();
                    }
                    queue.push(nxt);
                }
            }
        }
    }
    return null;
}

function fmt(path) {
    if (path === null) return "null";
    return "[" + path.map(w => `"${w}"`).join(", ") + "]";
}

function main() {
    console.log(fmt(ladder("dog", "cat", ["dot", "dop", "dat", "cat"])));
    console.log(fmt(ladder("dog", "cat", ["dot", "tod", "dat", "dar"])));
}

main();
