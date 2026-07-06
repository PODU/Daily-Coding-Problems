// Day 1775: Mastermind consistency. Brute-force every 6-digit code with distinct
// digits (P(10,6)=151200); a code is valid if it reproduces every guess's score.
// O(P(10,6) * G) time, O(1) extra space.
function consistent(guesses) {
  // guesses: object {guessNumber: score}
  const parsed = Object.entries(guesses).map(([g, s]) => [
    String(g).padStart(6, "0").split("").map(Number),
    s,
  ]);
  const code = new Array(6);
  const used = new Array(10).fill(false);

  function check() {
    for (const [digits, score] of parsed) {
      let m = 0;
      for (let i = 0; i < 6; i++) if (code[i] === digits[i]) m++;
      if (m !== score) return false;
    }
    return true;
  }

  function rec(pos) {
    if (pos === 6) return check();
    for (let d = 0; d < 10; d++) {
      if (used[d]) continue;
      used[d] = true;
      code[pos] = d;
      if (rec(pos + 1)) { used[d] = false; return true; }
      used[d] = false;
    }
    return false;
  }
  return rec(0);
}

console.log(consistent({ 175286: 2, 293416: 3, 654321: 0 }) ? "True" : "False");
console.log(consistent({ 123456: 4, 345678: 4, 567890: 4 }) ? "True" : "False");
