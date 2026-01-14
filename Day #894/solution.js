// HitCounter: maintain timestamps in a sorted array via binary-search insert. record O(n) shift,
// total = length O(1), range = upperBound - lowerBound via binary search O(log n).
// Limited-memory follow-up: bucket hits by time window (circular buffer of fixed size)
// so memory stays O(window) instead of O(#hits), trading exact old-range queries for recency.

class HitCounter {
    constructor() {
        this.ts = [];
    }

    record(t) {
        const pos = this._lowerBound(t);
        this.ts.splice(pos, 0, t);
    }

    total() {
        return this.ts.length;
    }

    range(lower, upper) {
        return this._upperBound(upper) - this._lowerBound(lower);
    }

    // first index with ts[i] >= key
    _lowerBound(key) {
        let lo = 0, hi = this.ts.length;
        while (lo < hi) {
            const mid = (lo + hi) >> 1;
            if (this.ts[mid] < key) lo = mid + 1;
            else hi = mid;
        }
        return lo;
    }

    // first index with ts[i] > key
    _upperBound(key) {
        let lo = 0, hi = this.ts.length;
        while (lo < hi) {
            const mid = (lo + hi) >> 1;
            if (this.ts[mid] <= key) lo = mid + 1;
            else hi = mid;
        }
        return lo;
    }
}

function main() {
    const hc = new HitCounter();
    hc.record(1);
    hc.record(2);
    hc.record(3);
    hc.record(2);
    console.log(hc.total());      // 4
    console.log(hc.range(2, 3));  // 3
}

main();
