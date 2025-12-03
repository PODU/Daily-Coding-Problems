// 2-SAT: implication graph + iterative Tarjan SCC. Sat iff no var shares SCC with its negation.
// Time O(n + m), Space O(n + m).
'use strict';

function twoSat(numVars, clauses) {
  const N = 2 * numVars;
  const g = Array.from({ length: N }, () => []);
  const node = (v, sign) => 2 * v + (sign ? 0 : 1);
  const neg = (x) => x ^ 1;

  for (const [[v1, s1], [v2, s2]] of clauses) {
    const x = node(v1, s1), y = node(v2, s2);
    g[neg(x)].push(y);
    g[neg(y)].push(x);
  }

  const idx = new Array(N).fill(-1);
  const low = new Array(N).fill(0);
  const comp = new Array(N).fill(-1);
  const onstk = new Array(N).fill(false);
  const stk = [];
  let cnt = 0, cc = 0;

  function tarjan(start) {
    const work = [[start, 0]];
    while (work.length) {
      const frame = work[work.length - 1];
      const v = frame[0], pi = frame[1];
      if (pi === 0) {
        idx[v] = low[v] = cnt++;
        stk.push(v);
        onstk[v] = true;
      }
      let recurse = false;
      let i = pi;
      for (; i < g[v].length; i++) {
        const w = g[v][i];
        if (idx[w] === -1) {
          frame[1] = i + 1;
          work.push([w, 0]);
          recurse = true;
          break;
        } else if (onstk[w]) {
          low[v] = Math.min(low[v], idx[w]);
        }
      }
      if (recurse) continue;
      if (low[v] === idx[v]) {
        while (true) {
          const w = stk.pop();
          onstk[w] = false;
          comp[w] = cc;
          if (w === v) break;
        }
        cc++;
      }
      work.pop();
      if (work.length) {
        const pv = work[work.length - 1][0];
        low[pv] = Math.min(low[pv], low[v]);
      }
    }
  }

  for (let i = 0; i < N; i++) if (idx[i] === -1) tarjan(i);

  const assign = new Array(numVars).fill(false);
  for (let v = 0; v < numVars; v++) {
    if (comp[2 * v] === comp[2 * v + 1]) return null;
    assign[v] = comp[2 * v] < comp[2 * v + 1];
  }
  return assign;
}

function main() {
  // Variables a=0, b=1, c=2. sign true=positive, false=negated.
  // (!c OR b) AND (b OR c) AND (!b OR c) AND (!c OR !a)
  const clauses = [
    [[2, false], [1, true]],
    [[1, true], [2, true]],
    [[1, false], [2, true]],
    [[2, false], [0, false]],
  ];
  const assign = twoSat(3, clauses);
  if (assign === null) {
    console.log('UNSATISFIABLE');
    return;
  }
  const names = ['a', 'b', 'c'];
  console.log(names.map((nm, i) => `${nm} = ${assign[i] ? 'True' : 'False'}`).join(', '));
}

main();
