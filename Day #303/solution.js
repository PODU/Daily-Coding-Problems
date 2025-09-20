// Day 303: Angle between clock hands. O(1).
// Bonus: hands overlap (angle 0) every 12/11 hours (~65.45 min apart).
function clockAngle(h, m) {
  const hr = (h % 12) * 30.0 + m * 0.5;
  const mn = m * 6.0;
  let diff = Math.abs(hr - mn);
  diff = Math.min(diff, 360.0 - diff);
  return Math.round(diff);
}
console.log(clockAngle(3, 30)); // 75
