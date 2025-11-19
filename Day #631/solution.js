// Day 631: Skyline problem.
// Approach: sweep line over edges + active-height multiset (Map + tracked max).
// Time: O(n log n), Space: O(n).
function getSkyline(buildings) {
  const events = [];
  for (const [l, r, h] of buildings) {
    events.push([l, -h]); // start
    events.push([r, h]);  // end
  }
  events.sort((a, b) => (a[0] !== b[0] ? a[0] - b[0] : a[1] - b[1]));
  // multiset of heights via Map; recompute max lazily
  const cnt = new Map([[0, 1]]);
  const heights = [0];
  const res = [];
  let prev = 0;
  for (const [x, h] of events) {
    if (h < 0) {
      const k = -h;
      cnt.set(k, (cnt.get(k) || 0) + 1);
      heights.push(k);
    } else {
      cnt.set(h, cnt.get(h) - 1);
    }
    while (heights.length && (cnt.get(Math.max(...heights)) || 0) === 0) {
      // remove stale max
      const m = Math.max(...heights);
      heights.splice(heights.indexOf(m), 1);
    }
    const cur = heights.length ? Math.max(...heights) : 0;
    if (cur !== prev) {
      res.push([x, cur]);
      prev = cur;
    }
  }
  return res;
}

const buildings = [[0, 15, 3], [4, 11, 5], [19, 23, 4]];
const sky = getSkyline(buildings);
console.log("[" + sky.map(([x, h]) => `(${x}, ${h})`).join(", ") + "]");
