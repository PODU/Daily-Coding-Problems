// Largest rectangle of 1s: per-row histogram + largest-rectangle-in-histogram via stack.
// Time O(N*M), Space O(M).
function largestInHist(h) {
    let best = 0;
    const st = [];
    for (let i = 0; i <= h.length; i++) {
        const cur = i === h.length ? 0 : h[i];
        while (st.length && h[st[st.length - 1]] >= cur) {
            const height = h[st.pop()];
            const width = st.length ? i - st[st.length - 1] - 1 : i;
            best = Math.max(best, height * width);
        }
        st.push(i);
    }
    return best;
}

function main() {
    const mat = [[1,0,0,0],[1,0,1,1],[1,0,1,1],[0,1,0,0]];
    const m = mat[0].length;
    const h = new Array(m).fill(0);
    let best = 0;
    for (const row of mat) {
        for (let j = 0; j < m; j++) h[j] = row[j] ? h[j] + 1 : 0;
        best = Math.max(best, largestInHist(h));
    }
    console.log(best);
}

main();
