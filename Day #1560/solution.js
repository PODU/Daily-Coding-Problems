// Greedy interval stabbing: sort by right endpoint; for the smallest uncovered right,
// pick point = max left among intervals containing it, cover them all. Time O(n^2), Space O(n).
function stabPoints(intervals) {
  const iv = [...intervals].sort((a, b) => a[1] - b[1]);
  const n = iv.length;
  const covered = new Array(n).fill(false);
  const points = [];
  for (let i = 0; i < n; i++) {
    if (covered[i]) continue;
    const r = iv[i][1];
    let point = -Infinity;
    for (let j = i; j < n; j++)
      if (!covered[j] && iv[j][0] <= r) point = Math.max(point, iv[j][0]);
    points.push(point);
    for (let j = i; j < n; j++)
      if (!covered[j] && iv[j][0] <= point && point <= iv[j][1]) covered[j] = true;
  }
  return points;
}

const pts = stabPoints([[0, 3], [2, 6], [3, 4], [6, 9]]);
console.log("{" + pts.join(", ") + "}");
