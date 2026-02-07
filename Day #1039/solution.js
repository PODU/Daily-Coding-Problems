// Fenwick/BIT over 24 hours: point update, prefix-sum range query.
// update O(log n), query O(log n).

class BIT {
    constructor(n) {
        this.n = n;
        this.tree = new Array(n + 1).fill(0);
    }
    update(hour, value) {
        for (let i = hour + 1; i <= this.n; i += i & (-i)) this.tree[i] += value;
    }
    prefix(idx) { // sum of [0..idx]
        let s = 0;
        for (let i = idx + 1; i > 0; i -= i & (-i)) s += this.tree[i];
        return s;
    }
    query(start, end) { // inclusive
        return this.prefix(end) - (start > 0 ? this.prefix(start - 1) : 0);
    }
}

function main() {
    const bit = new BIT(24);
    bit.update(0, 5);
    bit.update(3, 10);
    bit.update(23, 2);
    bit.update(3, 1);
    console.log("query(0, 3) = " + bit.query(0, 3));
    console.log("query(0, 23) = " + bit.query(0, 23));
    console.log("query(4, 23) = " + bit.query(4, 23));
    console.log("query(3, 3) = " + bit.query(3, 3));
}

main();
