// Domino + Tromino tiling of 2xN: f(n)=2*f(n-1)+f(n-3), f(0)=1,f(1)=1,f(2)=2.
// O(n) time, O(1) space.

function tilings(n) {
    if (n === 0) return 1;
    if (n === 1) return 1;
    if (n === 2) return 2;
    let a = 1, b = 1, c = 2, cur = c;
    for (let i = 3; i <= n; i++) {
        cur = 2 * c + a;
        a = b; b = c; c = cur;
    }
    return cur;
}

function main() {
    const N = 4;
    console.log(tilings(N)); // 11
}

main();
