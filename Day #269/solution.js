// Day 269: Push dominoes simulation via force/two-pointer scan.
// Left-to-right add +force from R, right-to-left add -force from L, sign decides. Time O(n), Space O(n).

function pushDominoes(s) {
    const n = s.length;
    const force = new Array(n).fill(0);
    let f = 0;
    for (let i = 0; i < n; i++) {              // rightward push
        if (s[i] === 'R') f = n;
        else if (s[i] === 'L') f = 0;
        else f = Math.max(f - 1, 0);
        force[i] += f;
    }
    f = 0;
    for (let i = n - 1; i >= 0; i--) {         // leftward push
        if (s[i] === 'L') f = n;
        else if (s[i] === 'R') f = 0;
        else f = Math.max(f - 1, 0);
        force[i] -= f;
    }
    return force.map(v => (v > 0 ? 'R' : v < 0 ? 'L' : '.')).join('');
}

console.log(pushDominoes(".L.R....L"));
console.log(pushDominoes("..R...L.L"));
