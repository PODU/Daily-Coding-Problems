// Day 1837: Stable marriage via Gale-Shapley (men propose). Always yields a stable matching.
// Time O(N^2), Space O(N^2) for preference/ranking tables.

function stableMatch(guyPref, galPref) {
  const galRank = {};
  for (const gal in galPref) {
    galRank[gal] = {};
    galPref[gal].forEach((g, i) => (galRank[gal][g] = i));
  }
  const next = {};
  const galPartner = {};
  const free = [];
  for (const guy in guyPref) {
    free.push(guy);
    next[guy] = 0;
  }
  while (free.length) {
    const guy = free.shift();
    const gal = guyPref[guy][next[guy]++];
    const cur = galPartner[gal];
    if (cur === undefined) {
      galPartner[gal] = guy;
    } else if (galRank[gal][guy] < galRank[gal][cur]) {
      galPartner[gal] = guy;
      free.push(cur);
    } else {
      free.push(guy);
    }
  }
  const guyPartner = {};
  for (const gal in galPartner) guyPartner[galPartner[gal]] = gal;
  return guyPartner;
}

function main() {
  const guyPreferences = {
    andrew: ["caroline", "abigail", "betty"],
    bill: ["caroline", "betty", "abigail"],
    chester: ["betty", "caroline", "abigail"],
  };
  const galPreferences = {
    abigail: ["andrew", "bill", "chester"],
    betty: ["bill", "andrew", "chester"],
    caroline: ["bill", "chester", "andrew"],
  };
  const match = stableMatch(guyPreferences, galPreferences);
  for (const guy of Object.keys(match).sort()) console.log(`${guy} -> ${match[guy]}`);
}

main();
