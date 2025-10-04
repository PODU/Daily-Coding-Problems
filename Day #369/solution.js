// Day 369: Stock price tracker.
// ts2price maps timestamp->price; a sorted array of prices (binary insert) gives
// min/max at the ends, running sum+count give average. add/update/remove O(n).
'use strict';

function lowerBound(arr, x) {
  let lo = 0, hi = arr.length;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (arr[mid] < x) lo = mid + 1;
    else hi = mid;
  }
  return lo;
}

class StockTracker {
  constructor() {
    this.ts2price = new Map();
    this.prices = [];
    this.sum = 0;
  }
  add(ts, price) {
    this.ts2price.set(ts, price);
    this.prices.splice(lowerBound(this.prices, price), 0, price);
    this.sum += price;
  }
  update(ts, price) {
    this.remove(ts);
    this.add(ts, price);
  }
  remove(ts) {
    if (!this.ts2price.has(ts)) return;
    const price = this.ts2price.get(ts);
    this.ts2price.delete(ts);
    this.prices.splice(lowerBound(this.prices, price), 1);
    this.sum -= price;
  }
  max() { return this.prices[this.prices.length - 1]; }
  min() { return this.prices[0]; }
  average() { return this.sum / this.prices.length; }
}

const s = new StockTracker();
s.add(1, 100); s.add(2, 200); s.add(3, 150);
console.log(`max=${s.max()} min=${s.min()} avg=${s.average().toFixed(1)}`);
s.update(2, 50);
console.log(`max=${s.max()} min=${s.min()} avg=${s.average().toFixed(1)}`);
s.remove(3);
console.log(`max=${s.max()} min=${s.min()} avg=${s.average().toFixed(1)}`);
