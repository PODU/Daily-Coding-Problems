// Day 286: Skyline problem.
// Sweep line over events; a sorted multiset (map) tracks active heights.
// Time: O(n log n), Space: O(n).
function getSkyline(buildings) {
  const events = [];
  for (const [l, r, h] of buildings) {
    events.push([l, -h]);
    events.push([r, h]);
  }
  events.sort((a, b) => (a[0] !== b[0] ? a[0] - b[0] : a[1] - b[1]));
  const live = new Map([[0, 1]]);
  const sortedKeys = () => [...live.keys()].sort((a, b) => b - a);
  let prev = 0;
  const res = [];
  for (const [x, h] of events) {
    if (h < 0) live.set(-h, (live.get(-h) || 0) + 1);
    else {
      const c = live.get(h);
      if (c === 1) live.delete(h); else live.set(h, c - 1);
    }
    const cur = sortedKeys()[0];
    if (cur !== prev) { res.push([x, cur]); prev = cur; }
  }
  return res;
}

const buildings = [[0, 15, 3], [4, 11, 5], [19, 23, 4]];
const sky = getSkyline(buildings);
console.log("[" + sky.map(p => `(${p[0]}, ${p[1]})`).join(", ") + "]");
