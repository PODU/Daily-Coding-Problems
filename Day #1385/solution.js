// Spiral matrix traversal using four boundary pointers (top,bottom,left,right). O(N*M) time, O(1) extra space.
function spiral(mat) {
    let top = 0, bottom = mat.length - 1;
    let left = 0, right = mat[0].length - 1;
    const out = [];
    while (top <= bottom && left <= right) {
        for (let j = left; j <= right; j++) out.push(mat[top][j]);
        top++;
        for (let i = top; i <= bottom; i++) out.push(mat[i][right]);
        right--;
        if (top <= bottom) {
            for (let j = right; j >= left; j--) out.push(mat[bottom][j]);
            bottom--;
        }
        if (left <= right) {
            for (let i = bottom; i >= top; i--) out.push(mat[i][left]);
            left++;
        }
    }
    return out;
}

const mat = [[1,2,3,4,5],[6,7,8,9,10],[11,12,13,14,15],[16,17,18,19,20]];
console.log(spiral(mat).join("\n"));
