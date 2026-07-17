// Word circle = Eulerian circuit in graph where each word is an edge first->last char.
// Check in==out degrees, then Hierholzer to build chain. O(V+E).
function solve(words) {
  const adj = new Map();
  const indeg = new Map(), outdeg = new Map(), idx = new Map();
  const inc = (m, k) => m.set(k, (m.get(k) || 0) + 1);
  for (const w of words) {
    const a = w[0], b = w[w.length - 1];
    if (!adj.has(a)) adj.set(a, []);
    adj.get(a).push([w, b]);
    inc(outdeg, a);
    inc(indeg, b);
  }
  const nodes = new Set([...indeg.keys(), ...outdeg.keys()]);
  for (const c of nodes) {
    if ((indeg.get(c) || 0) !== (outdeg.get(c) || 0)) return "No circle";
  }

  const start = words[0][0];
  const st = [start];
  const edgeStack = [];
  const circuit = [];
  while (st.length) {
    const u = st[st.length - 1];
    const lst = adj.get(u) || [];
    const i = idx.get(u) || 0;
    if (i < lst.length) {
      idx.set(u, i + 1);
      const [w, v] = lst[i];
      st.push(v);
      edgeStack.push(w);
    } else {
      st.pop();
      if (edgeStack.length) circuit.push(edgeStack.pop());
    }
  }
  if (circuit.length !== words.length) return "No circle";
  circuit.reverse();
  return circuit.join(" --> ") + " --> " + circuit[0];
}

const words = ["chair", "height", "racket", "touch", "tunic"];
console.log(solve(words));
