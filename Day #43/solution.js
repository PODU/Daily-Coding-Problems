// Max Stack: main stack + auxiliary stack holding running max. All ops O(1).
// Time O(1) per op; Space O(n).
class MaxStack {
  constructor() {
    this._data = [];
    this._maxes = [];
  }
  push(v) {
    this._data.push(v);
    this._maxes.push(this._maxes.length === 0 ? v : Math.max(v, this._maxes[this._maxes.length - 1]));
  }
  pop() {
    if (this._data.length === 0) return null;
    this._maxes.pop();
    return this._data.pop();
  }
  max() {
    return this._maxes.length === 0 ? null : this._maxes[this._maxes.length - 1];
  }
}

const s = new MaxStack();
for (const v of [3, 1, 5, 4]) {
  s.push(v);
  console.log(`push ${v} -> max=${s.max()}`);
}
console.log(`pop -> ${s.pop()}, max=${s.max()}`);
console.log(`pop -> ${s.pop()}, max=${s.max()}`);
