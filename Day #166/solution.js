// 2D iterator: track row/col indices, advance() skips empty subarrays. O(1) amortized per next/hasNext, O(1) extra space.

class Iterator2D {
    constructor(data) {
        this.data = data;
        this.row = 0;
        this.col = 0;
        this._advance();
    }
    _advance() {
        while (this.row < this.data.length && this.col >= this.data[this.row].length) {
            this.row++;
            this.col = 0;
        }
    }
    hasNext() {
        this._advance();
        return this.row < this.data.length;
    }
    next() {
        if (!this.hasNext()) throw new Error("no more elements");
        return this.data[this.row][this.col++];
    }
}

function main() {
    const arr = [[1, 2], [3], [], [4, 5, 6]];
    const it = new Iterator2D(arr);
    const out = [];
    while (it.hasNext()) out.push(it.next());
    console.log(out.join(", "));
}

main();
