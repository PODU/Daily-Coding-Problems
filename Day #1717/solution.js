// Day 1717: Fully justify text into lines of length k.
// Greedy line packing + even space distribution (extras from left).
// Time: O(total characters), Space: O(output).

function justify(words, k) {
    const lines = [];
    const n = words.length;
    let i = 0;
    while (i < n) {
        let j = i, lineLen = words[i].length;
        while (j + 1 < n && lineLen + 1 + words[j + 1].length <= k) {
            j++;
            lineLen += 1 + words[j].length;
        }
        const cnt = j - i + 1;
        let wordChars = 0;
        for (let t = i; t <= j; t++) wordChars += words[t].length;
        let line;
        if (cnt === 1) {
            line = words[i] + " ".repeat(k - words[i].length);
        } else {
            const gaps = cnt - 1;
            const totalSpaces = k - wordChars;
            const base = Math.floor(totalSpaces / gaps);
            const extra = totalSpaces % gaps;
            const parts = [];
            for (let t = i; t <= j; t++) {
                parts.push(words[t]);
                if (t < j) {
                    const sp = base + (t - i < extra ? 1 : 0);
                    parts.push(" ".repeat(sp));
                }
            }
            line = parts.join("");
        }
        lines.push(line);
        i = j + 1;
    }
    return lines;
}

const words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"];
for (const l of justify(words, 16)) console.log('"' + l + '"');
