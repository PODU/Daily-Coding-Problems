// Top-k website pairs by Jaccard similarity over user sets; sort desc, tie-break alpha.
// O(W^2 * U) to compare pairs. Output formatted as Python tuple list.
'use strict';

function topKPairs(visits, k) {
    const users = new Map();
    for (const [site, user] of visits) {
        if (!users.has(site)) users.set(site, new Set());
        users.get(site).add(user);
    }

    const sites = [...users.keys()].sort();
    const results = [];
    for (let i = 0; i < sites.length; i++)
        for (let j = i + 1; j < sites.length; j++) {
            const A = users.get(sites[i]), B = users.get(sites[j]);
            let inter = 0;
            for (const u of A) if (B.has(u)) inter++;
            const uni = A.size + B.size - inter;
            results.push([uni ? inter / uni : 0, sites[i], sites[j]]);
        }

    results.sort((x, y) => y[0] - x[0] || (x[1] < y[1] ? -1 : x[1] > y[1] ? 1 : (x[2] < y[2] ? -1 : 1)));
    return results.slice(0, k).map(r => [r[1], r[2]]);
}

function main() {
    const visits = [['a', 1], ['a', 3], ['a', 5], ['b', 2], ['b', 6],
        ['c', 1], ['c', 2], ['c', 3], ['c', 4], ['c', 5],
        ['d', 4], ['d', 5], ['d', 6], ['d', 7],
        ['e', 1], ['e', 3], ['e', 5], ['e', 6]];
    const res = topKPairs(visits, 1);
    console.log("[" + res.map(p => `('${p[0]}', '${p[1]}')`).join(", ") + "]");
}

main();
