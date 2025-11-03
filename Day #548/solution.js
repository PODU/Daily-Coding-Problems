// Clock hand angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take abs diff, fold >180, round. O(1) time/space.
// Fun fact: the hands overlap (angle 0) 22 times a day, not 24.
'use strict';

function clockAngle(t) {
  const [hh, mm] = t.split(':').map(Number);
  const minute = mm * 6;
  const hour = (hh % 12) * 30 + mm * 0.5;
  let diff = Math.abs(hour - minute);
  if (diff > 180) diff = 360 - diff;
  return Math.round(diff);
}

console.log(clockAngle("12:00"));
console.log(clockAngle("3:30"));
console.log(clockAngle("9:00"));
