// Word break reconstruction via DP with backpointers. O(n^2) time, O(n) space.

function wordBreak(s, words) {
  const dict = new Set(words);
  const n = s.length;
  const back = new Array(n + 1).fill(-2); // -2 unreachable
  back[0] = -1;
  for (let i = 1; i <= n; i++)
    for (let j = 0; j < i; j++)
      if (back[j] !== -2 && dict.has(s.slice(j, i))) { back[i] = j; break; }
  if (back[n] === -2) return null;
  const res = [];
  for (let i = n; i > 0; i = back[i]) res.push(s.slice(back[i], i));
  return res.reverse();
}

const words = ['quick', 'brown', 'the', 'fox'];
const s = "thequickbrownfox";
const res = wordBreak(s, words);
console.log("[" + res.map((w) => `'${w}'`).join(", ") + "]");
