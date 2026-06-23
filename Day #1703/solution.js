// Prefix sums: precompute prefix[k]=sum(L[0:k]); sum(i,j)=prefix[j]-prefix[i].
// Preprocess O(n), query O(1), Space O(n).
class RangeSum {
    constructor(L) {
        this.prefix = new Array(L.length + 1).fill(0);
        for (let k = 0; k < L.length; k++) this.prefix[k + 1] = this.prefix[k] + L[k];
    }
    sum(i, j) { return this.prefix[j] - this.prefix[i]; }
}

function main() {
    const L = [1, 2, 3, 4, 5];
    const rs = new RangeSum(L);
    console.log(rs.sum(1, 3));
}

main();
