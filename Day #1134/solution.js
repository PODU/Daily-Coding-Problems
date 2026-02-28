// Running median with two binary heaps (max-heap low, min-heap high). O(log n) per insert.

class Heap {
  constructor(cmp) {
    this.a = [];
    this.cmp = cmp;
  }
  size() { return this.a.length; }
  peek() { return this.a[0]; }
  push(v) {
    const a = this.a;
    a.push(v);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (this.cmp(a[i], a[p]) < 0) { [a[i], a[p]] = [a[p], a[i]]; i = p; }
      else break;
    }
  }
  pop() {
    const a = this.a;
    const top = a[0];
    const last = a.pop();
    if (a.length > 0) {
      a[0] = last;
      let i = 0;
      const n = a.length;
      while (true) {
        let l = 2 * i + 1, r = 2 * i + 2, s = i;
        if (l < n && this.cmp(a[l], a[s]) < 0) s = l;
        if (r < n && this.cmp(a[r], a[s]) < 0) s = r;
        if (s === i) break;
        [a[i], a[s]] = [a[s], a[i]];
        i = s;
      }
    }
    return top;
  }
}

function fmt(x) {
  return Number.isInteger(x) ? String(x) : String(x);
}

function runningMedian(nums) {
  const low = new Heap((a, b) => b - a);  // max-heap
  const high = new Heap((a, b) => a - b); // min-heap
  const out = [];
  for (const x of nums) {
    if (low.size() === 0 || x <= low.peek()) low.push(x);
    else high.push(x);
    if (low.size() > high.size() + 1) high.push(low.pop());
    else if (high.size() > low.size()) low.push(high.pop());

    let med;
    if (low.size() === high.size()) med = (low.peek() + high.peek()) / 2;
    else med = low.peek();
    out.push(fmt(med));
  }
  return out;
}

for (const line of runningMedian([2, 1, 5, 7, 2, 0, 5])) {
  console.log(line);
}
