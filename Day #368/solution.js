// Day 368: KV store with update/get/maxKey(val).
// kv maps key->value; byVal maps value->sorted array of keys (binary insert), so
// maxKey is the last element. update O(log n), get O(1), maxKey O(1).
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

class KVStore {
  constructor() {
    this.kv = new Map();
    this.byVal = new Map();
  }

  update(key, val) {
    if (this.kv.has(key)) {
      const old = this.kv.get(key);
      const lst = this.byVal.get(old);
      lst.splice(lowerBound(lst, key), 1);
      if (lst.length === 0) this.byVal.delete(old);
    }
    this.kv.set(key, val);
    if (!this.byVal.has(val)) this.byVal.set(val, []);
    const lst = this.byVal.get(val);
    lst.splice(lowerBound(lst, key), 0, key);
  }

  get(key) {
    return this.kv.has(key) ? this.kv.get(key) : null;
  }

  maxKey(val) {
    const lst = this.byVal.get(val);
    return lst && lst.length ? lst[lst.length - 1] : null;
  }
}

const kv = new KVStore();
kv.update(1, 1);
kv.update(2, 1);
console.log(kv.maxKey(1)); // 2
