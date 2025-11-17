// Stable Marriage (Gale-Shapley, men proposing). Each free man proposes down his list.
// Time: O(N^2), Space: O(N^2) for preference rank tables.
'use strict';

function galeShapley(guy, gal) {
  const men = Object.keys(guy);
  const rank = {};
  for (const w of Object.keys(gal)) {
    rank[w] = {};
    gal[w].forEach((m, i) => { rank[w][m] = i; });
  }
  const next = {};
  men.forEach(m => { next[m] = 0; });
  const husband = {};
  const free = [...men];

  while (free.length) {
    const m = free.shift();
    const w = guy[m][next[m]++];
    if (!(w in husband)) {
      husband[w] = m;
    } else {
      const cur = husband[w];
      if (rank[w][m] < rank[w][cur]) { husband[w] = m; free.push(cur); }
      else free.push(m);
    }
  }

  const result = {};
  const wife = {};
  for (const w of Object.keys(husband)) wife[husband[w]] = w;
  men.forEach(m => { result[m] = wife[m]; });
  return result;
}

if (require.main === module) {
  const guy_preferences = {
    andrew:  ['caroline', 'abigail', 'betty'],
    bill:    ['caroline', 'betty', 'abigail'],
    chester: ['betty', 'caroline', 'abigail'],
  };
  const gal_preferences = {
    abigail:  ['andrew', 'bill', 'chester'],
    betty:    ['bill', 'andrew', 'chester'],
    caroline: ['bill', 'chester', 'andrew'],
  };
  const res = galeShapley(guy_preferences, gal_preferences);
  const body = Object.keys(res).map(m => `'${m}': '${res[m]}'`).join(', ');
  console.log(`{${body}}`);
}
