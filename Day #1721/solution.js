// Reverse words while keeping delimiters in place: split into word/delimiter tokens,
// reverse only the word list, re-emit in original token order. Time O(n), Space O(n).
"use strict";

function reverseWords(s, delims) {
    const tokens = []; // {text, isWord}
    let cur = "";
    for (const c of s) {
        if (delims.has(c)) {
            if (cur.length > 0) { tokens.push({ text: cur, isWord: true }); cur = ""; }
            tokens.push({ text: c, isWord: false });
        } else {
            cur += c;
        }
    }
    if (cur.length > 0) tokens.push({ text: cur, isWord: true });

    const words = tokens.filter(t => t.isWord).map(t => t.text);
    words.reverse();

    let res = "";
    let wi = 0;
    for (const t of tokens) res += t.isWord ? words[wi++] : t.text;
    return res;
}

const delims = new Set(["/", ":"]);
console.log(reverseWords("hello/world:here", delims));
console.log(reverseWords("hello/world:here/", delims));
console.log(reverseWords("hello//world:here", delims));
