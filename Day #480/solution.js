// Word break reconstruction via memoized DP: for each suffix, try each prefix
// word and recurse. Time: O(n^2 * L) with memo, Space: O(n^2).

function reconstruct(words, s) {
  const dict = new Set(words);
  const memo = new Map();

  function solve(start) {
    if (start === s.length) return [];
    if (memo.has(start)) return memo.get(start);
    for (let end = start + 1; end <= s.length; end++) {
      const word = s.slice(start, end);
      if (dict.has(word)) {
        const rest = solve(end);
        if (rest !== null) {
          const res = [word, ...rest];
          memo.set(start, res);
          return res;
        }
      }
    }
    memo.set(start, null);
    return null;
  }

  return solve(0);
}

function fmt(res) {
  if (res === null) return "null";
  return "[" + res.map((w) => `'${w}'`).join(", ") + "]";
}

function main() {
  console.log(fmt(reconstruct(["quick", "brown", "the", "fox"], "thequickbrownfox")));
  console.log(fmt(reconstruct(["bed", "bath", "bedbath", "and", "beyond"], "bedbathandbeyond")));
}

main();
