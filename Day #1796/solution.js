// Uniform random in [0,n-1] excluding values in l: precompute sorted allowed array, pick random index. O(n) build, O(allowed) space.
class RandomPicker {
  constructor(n, l) {
    const ex = new Set(l);
    this.allowed = [];
    for (let i = 0; i < n; i++) if (!ex.has(i)) this.allowed.push(i);
  }
  pick() {
    return this.allowed[Math.floor(Math.random() * this.allowed.length)];
  }
}

const rp = new RandomPicker(10, [2, 3, 5, 7]);
const aset = new Set(rp.allowed);
console.log("allowed=[" + rp.allowed.join(", ") + "]");
let ok = true;
for (let i = 0; i < 1000; i++) if (!aset.has(rp.pick())) ok = false;
console.log("sample in allowed: " + ok);
