// rand5 from rand7 via rejection sampling: call rand7() until <=5. Uniform 1..5. Time O(1) expected, Space O(1).
'use strict';

function rand7() {
    return Math.floor(Math.random() * 7) + 1;
}

function rand5() {
    let x;
    do { x = rand7(); } while (x > 5);
    return x;
}

function main() {
    const trials = 100000;
    const counts = new Array(6).fill(0);
    for (let i = 0; i < trials; i++) {
        const v = rand5();
        if (v < 1 || v > 5) throw new Error('out of range');
        counts[v]++;
    }
    const expected = trials / 5.0;
    for (let v = 1; v <= 5; v++) {
        if (Math.abs(counts[v] - expected) >= expected * 0.1)
            throw new Error('not uniform');
    }
    console.log('rand5 OK');
}

main();
