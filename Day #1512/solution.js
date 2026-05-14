// First recurring character: scan L->R, track seen set; first char already seen wins.
// O(n) time, O(alphabet) space.

function firstRecurring(s) {
    const seen = new Set();
    for (const c of s) {
        if (seen.has(c)) return c;
        seen.add(c);
    }
    return null;
}

const r = firstRecurring("acbbac");
console.log(r === null ? "null" : r);
