// Palindrome pairs: hash map of reversed words; for each word check prefix/suffix palindrome splits. O(n*k^2) time, O(n*k) space.

function isPalin(s, i, j) {
    while (i < j) {
        if (s[i] !== s[j]) return false;
        i++; j--;
    }
    return true;
}

function palindromePairs(words) {
    const rev = new Map();
    words.forEach((w, i) => rev.set(w.split("").reverse().join(""), i));

    const seen = new Set();
    const res = [];
    const add = (a, b) => {
        const key = a + "," + b;
        if (!seen.has(key)) { seen.add(key); res.push([a, b]); }
    };

    words.forEach((w, i) => {
        const n = w.length;
        for (let cut = 0; cut <= n; cut++) {
            if (isPalin(w, 0, cut - 1)) {
                const suf = w.slice(cut);
                if (rev.has(suf) && rev.get(suf) !== i) add(rev.get(suf), i);
            }
            if (cut < n && isPalin(w, cut, n - 1)) {
                const pre = w.slice(0, cut);
                if (rev.has(pre) && rev.get(pre) !== i) add(i, rev.get(pre));
            }
        }
    });

    res.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
    return res;
}

function main() {
    const words = ["code", "edoc", "da", "d"];
    const pairs = palindromePairs(words);
    console.log("[" + pairs.map(([a, b]) => `(${a}, ${b})`).join(", ") + "]");
}

main();
