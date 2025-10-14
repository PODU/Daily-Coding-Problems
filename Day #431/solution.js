// Day 431: Sentence validator via finite-state-machine scan over codepoints.
// Mirrors regex ^[A-Z][a-z]*([,;:]? [a-z]+)*[,;:]?[.?!‽]$ (no backtracking needed).
// O(n) time, O(n) space per sentence.
function isSep(c)  { return c === ',' || c === ';' || c === ':'; }
function isTerm(c) { return c === '.' || c === '?' || c === '!' || c === '‽'; }
function isLow(c)  { return c >= 'a' && c <= 'z'; }

function isValidSentence(s) {
    const a = Array.from(s);          // iterate by Unicode codepoint
    const n = a.length;
    if (n === 0) return false;
    if (!(a[0] >= 'A' && a[0] <= 'Z')) return false;
    let i = 1;
    while (i < n && isLow(a[i])) i++;
    while (true) {
        let j = i;
        if (j < n && isSep(a[j])) j++;
        if (j < n && a[j] === ' ') {
            j++;
            if (j < n && isLow(a[j])) {
                while (j < n && isLow(a[j])) j++;
                i = j;
                continue;
            }
        }
        break;
    }
    if (i < n && isSep(a[i])) i++;
    return i === n - 1 && isTerm(a[i]);
}

const tests = ["The quick brown fox.", "hello world.", "Hello  world.",
               "Hello world", "Hi there, friend!"];
for (const t of tests)
    console.log((isValidSentence(t) ? "Valid: " : "Invalid: ") + t);
