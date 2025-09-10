// Words form a circle: model each word as a directed edge first->last char; an Eulerian circuit
// exists iff in-degree==out-degree for every node and edges form one connected component.
// Find the circuit with Hierholzer's algorithm. O(V + E) time and space.

function circleOrder(words) {
  const adj = new Map(); // first char -> [[lastChar, word], ...]
  const indeg = new Map(), outdeg = new Map();
  const nodes = new Set();
  const inc = (m, k) => m.set(k, (m.get(k) || 0) + 1);
  for (const w of words) {
    const a = w[0], b = w[w.length - 1];
    if (!adj.has(a)) adj.set(a, []);
    adj.get(a).push([b, w]);
    inc(outdeg, a); inc(indeg, b);
    nodes.add(a); nodes.add(b);
  }
  for (const c of nodes)
    if ((indeg.get(c) || 0) !== (outdeg.get(c) || 0)) return null;

  // Connectivity (undirected) over nodes with outgoing edges.
  const und = new Map();
  const addU = (a, b) => { if (!und.has(a)) und.set(a, []); und.get(a).push(b); };
  for (const [a, lst] of adj) for (const [b] of lst) { addU(a, b); addU(b, a); }
  const active = [...adj].filter(([, lst]) => lst.length).map(([a]) => a);
  if (active.length === 0) return null;
  const seen = new Set();
  const st = [active[0]];
  while (st.length) {
    const c = st.pop();
    if (seen.has(c)) continue;
    seen.add(c);
    for (const nb of und.get(c) || []) if (!seen.has(nb)) st.push(nb);
  }
  if (active.some((c) => !seen.has(c))) return null;

  // Hierholzer starting from first word's first char.
  const start = words[0][0];
  const ptr = new Map();
  const nodeStack = [start];
  const edgeStack = [];
  const circuit = [];
  while (nodeStack.length) {
    const v = nodeStack[nodeStack.length - 1];
    const lst = adj.get(v) || [];
    const p = ptr.get(v) || 0;
    if (p < lst.length) {
      ptr.set(v, p + 1);
      const [w, word] = lst[p];
      nodeStack.push(w);
      edgeStack.push(word);
    } else {
      nodeStack.pop();
      if (edgeStack.length) circuit.push(edgeStack.pop());
    }
  }
  if (circuit.length !== words.length) return null;
  circuit.reverse();
  return circuit;
}

function main() {
  const words = ["chair", "height", "racket", "touch", "tunic"];
  const order = circleOrder(words);
  if (order) {
    console.log(order.join(" --> ") + " --> " + order[0]);
  } else {
    console.log("No circle possible");
  }
}

main();
