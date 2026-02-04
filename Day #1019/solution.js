// Rectangle intersection area: O(1) time, O(1) space.
// top_left is top y, height extends downward. x_overlap*y_overlap clamped at 0.
function intersectArea(a, b) {
    // rect = [left, top, width, height]
    const aRight = a[0] + a[2], bRight = b[0] + b[2];
    const aBottom = a[1] - a[3], bBottom = b[1] - b[3];
    const xo = Math.max(0, Math.min(aRight, bRight) - Math.max(a[0], b[0]));
    const yo = Math.max(0, Math.min(a[1], b[1]) - Math.max(aBottom, bBottom));
    return xo * yo;
}

const r1 = [1, 4, 3, 3];
const r2 = [0, 5, 4, 3];
console.log(intersectArea(r1, r2));
