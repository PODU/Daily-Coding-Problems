// Clock angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take min(diff,360-diff), round. O(1) time/space.
function clockAngle(hh, mm) {
  const minute = mm * 6;
  const hour = (hh % 12) * 30 + mm * 0.5;
  let diff = Math.abs(hour - minute);
  diff = Math.min(diff, 360 - diff);
  return Math.floor(diff + 0.5);
}

console.log(clockAngle(12, 30));
console.log(clockAngle(3, 15));
