// Simplified Elo: expected = 1/(1+10^((Rb-Ra)/400)); delta = round(K*(s-expected)); zero-sum transfer.
// Time O(1), Space O(1).
function main() {
  const Ra = 1200, Rb = 2000, K = 32;
  const expectedA = 1.0 / (1.0 + Math.pow(10.0, (Rb - Ra) / 400.0));
  const delta = Math.round(K * (1 - expectedA)); // A wins, s=1
  const newA = Ra + delta;
  const newB = Rb - delta;
  console.log(`Winner (${Ra}) -> ${newA}, Loser (${Rb}) -> ${newB}`);
}

main();
