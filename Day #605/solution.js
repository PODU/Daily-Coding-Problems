// h-index via counting/bucket sort: bucket citations capped at N, scan from high
// to low accumulating paper counts. Time O(n), Space O(n).
function hIndex(citations) {
  const n = citations.length;
  const bucket = new Array(n + 1).fill(0);
  for (const c of citations) bucket[Math.min(c, n)]++;
  let total = 0;
  for (let h = n; h >= 0; h--) {
    total += bucket[h];
    if (total >= h) return h;
  }
  return 0;
}

const citations = [4, 3, 0, 1, 5];
console.log(hIndex(citations));
