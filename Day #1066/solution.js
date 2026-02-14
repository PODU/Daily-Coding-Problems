// Gale-Shapley men-proposing stable matching. Time: O(N^2), Space: O(N^2).
function galeShapley(guyPref, galPref) {
    const men = Object.keys(guyPref);
    const galRank = {};
    for (const w in galPref) {
        galRank[w] = {};
        galPref[w].forEach((m, i) => { galRank[w][m] = i; });
    }
    const nextProposal = {};
    men.forEach(m => { nextProposal[m] = 0; });
    const womanPartner = {}, manPartner = {};
    const queue = [...men];
    while (queue.length) {
        const m = queue.shift();
        const w = guyPref[m][nextProposal[m]++];
        if (!(w in womanPartner)) {
            womanPartner[w] = m;
            manPartner[m] = w;
        } else {
            const m2 = womanPartner[w];
            if (galRank[w][m] < galRank[w][m2]) {
                womanPartner[w] = m;
                manPartner[m] = w;
                delete manPartner[m2];
                queue.push(m2);
            } else {
                queue.push(m);
            }
        }
    }
    return manPartner;
}

const guyPreferences = {
    andrew:  ['caroline', 'abigail', 'betty'],
    bill:    ['caroline', 'betty',   'abigail'],
    chester: ['betty',    'caroline','abigail'],
};
const galPreferences = {
    abigail:  ['andrew', 'bill',    'chester'],
    betty:    ['bill',   'andrew',  'chester'],
    caroline: ['bill',   'chester', 'andrew'],
};

const result = galeShapley(guyPreferences, galPreferences);
Object.keys(result).sort().forEach(m => console.log(`${m} -> ${result[m]}`));
