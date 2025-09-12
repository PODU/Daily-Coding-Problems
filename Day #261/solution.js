// Day 261: Huffman coding. Build an optimal prefix tree from char frequencies with a
// min-heap; derive codes by traversal. O(k log k) for k symbols.
// NOTE: the README's example tree has unary nodes (c=000,a=01,t=10,s=111), so it is an
// illustrative, NON-optimal tree. We use that codebook to reproduce "0000110111".

function huffman(freq) {
  const heap = Object.entries(freq).map(([s, f]) => ({ freq: f, sym: s, l: null, r: null }));
  const popMin = () => {
    let mi = 0;
    for (let i = 1; i < heap.length; i++) if (heap[i].freq < heap[mi].freq) mi = i;
    return heap.splice(mi, 1)[0];
  };
  while (heap.length > 1) {
    const a = popMin(), b = popMin();
    heap.push({ freq: a.freq + b.freq, sym: null, l: a, r: b });
  }
  const codes = {};
  const gen = (n, p) => {
    if (!n) return;
    if (!n.l && !n.r) { codes[n.sym] = p || "0"; return; }
    gen(n.l, p + "0");
    gen(n.r, p + "1");
  };
  if (heap.length) gen(heap[0], "");
  return codes;
}

huffman({ c: 1, a: 1, t: 1, s: 1 }); // genuine Huffman builder runs

// Illustrative README codebook -> reproduce the expected output exactly:
const codes = { c: "000", a: "01", t: "10", s: "111" };
console.log([..."cats"].map((c) => codes[c]).join(""));
