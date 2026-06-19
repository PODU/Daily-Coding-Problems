// Clock angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take min(diff,360-diff), round. O(1) time/space.
// Bonus: hands overlap (angle 0) 11 times per 12 hours, at t = (12/11)*k hours for k=0..10.

function clockAngle(hh, mm) {
  const minuteAngle = mm * 6.0;
  const hourAngle = (hh % 12) * 30.0 + mm * 0.5;
  const diff = Math.abs(hourAngle - minuteAngle);
  const angle = Math.min(diff, 360.0 - diff);
  return Math.round(angle);
}

const t = "3:15";
const [hh, mm] = t.split(":").map((x) => parseInt(x.trim(), 10));
console.log(clockAngle(hh, mm));
