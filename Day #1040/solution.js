// KMP string matching: build LPS failure array O(k), scan text O(N). Time O(N+k), Space O(k).
function kmpSearch(text, pat) {
    const n = text.length, k = pat.length;
    if (k === 0) return 0;
    const lps = new Array(k).fill(0);
    for (let i = 1, len = 0; i < k; ) {
        if (pat[i] === pat[len]) lps[i++] = ++len;
        else if (len) len = lps[len - 1];
        else lps[i++] = 0;
    }
    for (let i = 0, j = 0; i < n; ) {
        if (text[i] === pat[j]) { i++; j++; if (j === k) return i - j; }
        else if (j) j = lps[j - 1];
        else i++;
    }
    return -1;
}

const text = "abxabcabcaby";
const r1 = kmpSearch(text, "abcaby");
console.log(r1 === -1 ? "Not found" : "Found at index " + r1);
const r2 = kmpSearch(text, "xyz");
console.log(r2 === -1 ? "Not found" : "Found at index " + r2);
