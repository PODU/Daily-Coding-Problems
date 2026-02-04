// Clock angle: O(1) time, O(1) space.
// minute=mm*6, hour=(hh%12)*30+mm*0.5, diff=|h-m|, angle=min(diff,360-diff), rounded.
function clockAngle(hh, mm) {
    const minute = mm * 6.0;
    const hour = (hh % 12) * 30.0 + mm * 0.5;
    const diff = Math.abs(hour - minute);
    const angle = Math.min(diff, 360.0 - diff);
    return Math.round(angle);
}

const pad = (n) => String(n).padStart(2, "0");
console.log(`${pad(3)}:${pad(30)} -> ${clockAngle(3, 30)}`);
console.log(`${pad(12)}:${pad(0)} -> ${clockAngle(12, 0)}`);
