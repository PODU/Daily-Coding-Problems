// Day 371: Solve a system of addition-only equations over variables/constants.
// Build a linear system A x = b and run Gauss-Jordan elimination; unique integer
// solution -> mapping, otherwise null. Time O(eqs * vars^2).
'use strict';

function isNumber(t) {
  return /^-?\d+$/.test(t);
}

function solve(text) {
  const coeffsList = [];
  const bs = [];
  const varset = new Set();
  for (const raw of text.split('\n')) {
    const line = raw.trim();
    if (!line) continue;
    const [lhs, rhs] = line.split('=');
    const coeffs = {};
    let b = 0;
    for (const tok of lhs.split('+')) {
      const t = tok.trim();
      if (isNumber(t)) b -= parseInt(t, 10);
      else { coeffs[t] = (coeffs[t] || 0) + 1; varset.add(t); }
    }
    for (const tok of rhs.split('+')) {
      const t = tok.trim();
      if (isNumber(t)) b += parseInt(t, 10);
      else { coeffs[t] = (coeffs[t] || 0) - 1; varset.add(t); }
    }
    coeffsList.push(coeffs);
    bs.push(b);
  }

  const vars = [...varset].sort();
  const idx = {};
  vars.forEach((v, i) => (idx[v] = i));
  const n = vars.length;
  const aug = coeffsList.map((coeffs, r) => {
    const row = new Array(n + 1).fill(0);
    for (const v in coeffs) row[idx[v]] += coeffs[v];
    row[n] = bs[r];
    return row;
  });

  const m = aug.length;
  const pivotCols = [];
  let pr = 0;
  for (let col = 0; col < n; col++) {
    let sel = -1;
    for (let r = pr; r < m; r++) if (Math.abs(aug[r][col]) > 1e-9) { sel = r; break; }
    if (sel === -1) continue;
    [aug[pr], aug[sel]] = [aug[sel], aug[pr]];
    const pv = aug[pr][col];
    for (let c = 0; c <= n; c++) aug[pr][c] /= pv;
    for (let r = 0; r < m; r++) {
      if (r !== pr && Math.abs(aug[r][col]) > 1e-9) {
        const f = aug[r][col];
        for (let c = 0; c <= n; c++) aug[r][c] -= f * aug[pr][c];
      }
    }
    pivotCols.push(col);
    pr++;
  }
  for (let r = 0; r < m; r++) {
    let allZero = true;
    for (let c = 0; c < n; c++) if (Math.abs(aug[r][c]) > 1e-9) allZero = false;
    if (allZero && Math.abs(aug[r][n]) > 1e-9) return null;
  }
  if (pivotCols.length < n) return null;

  const sol = {};
  pivotCols.forEach((col, i) => (sol[vars[col]] = Math.round(aug[i][n])));
  return sol;
}

function fmt(sol) {
  if (sol === null) return 'null';
  const keys = Object.keys(sol).sort();
  return '{\n' + keys.map((k) => `  ${k}: ${sol[k]}`).join(',\n') + '\n}';
}

const text = 'y = x + 1\n5 = x + 3\n10 = z + y + 2';
console.log(fmt(solve(text)));
