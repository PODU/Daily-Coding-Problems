// Dominoes final state via two-pass force summation. Time: O(n), Space: O(n).
function dominoes(s) {
    const n = s.length;
    const forces = new Array(n).fill(0);
    // Left to right: R force propagates rightward
    let f = 0;
    for (let i = 0; i < n; i++) {
        if      (s[i]==='R') f = n;
        else if (s[i]==='L') f = 0;
        else f = Math.max(0, f-1);
        forces[i] += f;
    }
    // Right to left: L force propagates leftward (subtract)
    f = 0;
    for (let i = n-1; i >= 0; i--) {
        if      (s[i]==='L') f = n;
        else if (s[i]==='R') f = 0;
        else f = Math.max(0, f-1);
        forces[i] -= f;
    }
    return forces.map(v => v>0?'R':v<0?'L':'.').join('');
}

console.log(dominoes('.L.R....L'));
console.log(dominoes('..R...L.L'));
