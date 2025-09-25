// Gale-Shapley stable marriage, men propose; free man proposes down his list, women keep better partner.
// Time O(n^2), Space O(n^2).
function main() {
  const men = ["andrew", "bill", "chester"];
  const women = ["abigail", "betty", "caroline"];
  const guyPref = {
    andrew: ["caroline", "abigail", "betty"],
    bill: ["caroline", "betty", "abigail"],
    chester: ["betty", "caroline", "abigail"],
  };
  const galPref = {
    abigail: ["andrew", "bill", "chester"],
    betty: ["bill", "andrew", "chester"],
    caroline: ["bill", "chester", "andrew"],
  };

  const wrank = {};
  for (const w of women) {
    wrank[w] = {};
    galPref[w].forEach((m, i) => (wrank[w][m] = i));
  }

  const next = {};
  men.forEach((m) => (next[m] = 0));
  const partnerOf = {}; // woman -> man
  const freeMen = [...men];

  while (freeMen.length > 0) {
    const m = freeMen.shift();
    const w = guyPref[m][next[m]++];
    if (!(w in partnerOf)) {
      partnerOf[w] = m;
    } else {
      const cur = partnerOf[w];
      if (wrank[w][m] < wrank[w][cur]) {
        partnerOf[w] = m;
        freeMen.push(cur);
      } else {
        freeMen.push(m);
      }
    }
  }

  const manToWoman = {};
  for (const [woman, man] of Object.entries(partnerOf)) manToWoman[man] = woman;
  for (const m of men) console.log(`${m} - ${manToWoman[m]}`);
}

main();
