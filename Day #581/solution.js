// Rectangle intersection area via overlap of x and y ranges. Time O(1), Space O(1).
function intersectionArea(tl1, dims1, tl2, dims2) {
  const [left1, top1] = tl1;
  const right1 = left1 + dims1[0], bottom1 = top1 - dims1[1];
  const [left2, top2] = tl2;
  const right2 = left2 + dims2[0], bottom2 = top2 - dims2[1];
  const overlapW = Math.min(right1, right2) - Math.max(left1, left2);
  const overlapH = Math.min(top1, top2) - Math.max(bottom1, bottom2);
  return Math.max(0, overlapW) * Math.max(0, overlapH);
}

console.log(intersectionArea([1, 4], [3, 3], [0, 5], [4, 3]));
