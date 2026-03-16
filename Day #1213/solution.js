// Day 1213: Stable marriage via Gale-Shapley (men propose).
// Each free man proposes down his list; women keep their best suitor. Time O(N^2), Space O(N^2).
function stableMatch(guys, gals) {
  const rank = {};
  for (const [w, prefs] of Object.entries(gals)) {
    rank[w] = {};
    prefs.forEach((m, i) => (rank[w][m] = i));
  }
  const next = {};
  for (const m of Object.keys(guys)) next[m] = 0;
  const engaged = {}; // woman -> man
  const free = Object.keys(guys);
  while (free.length) {
    const m = free.shift();
    const w = guys[m][next[m]++];
    if (!(w in engaged)) engaged[w] = m;
    else {
      const cur = engaged[w];
      if (rank[w][m] < rank[w][cur]) { engaged[w] = m; free.push(cur); }
      else free.push(m);
    }
  }
  return engaged;
}

const guys = {
  andrew: ["caroline", "abigail", "betty"],
  bill: ["caroline", "betty", "abigail"],
  chester: ["betty", "caroline", "abigail"],
};
const gals = {
  abigail: ["andrew", "bill", "chester"],
  betty: ["bill", "andrew", "chester"],
  caroline: ["bill", "chester", "andrew"],
};
const match = stableMatch(guys, gals);
const byMan = {};
for (const [w, m] of Object.entries(match)) byMan[m] = w;
for (const man of Object.keys(guys).sort()) console.log(`${man} - ${byMan[man]}`);
// andrew - abigail / bill - caroline / chester - betty
