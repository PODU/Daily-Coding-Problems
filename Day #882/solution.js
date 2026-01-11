// 8-puzzle solver via A* with Manhattan-distance heuristic (optimal moves).
// State = 9-char string, 0 = blank. Time/space depend on solution depth.

const GOAL = "123456780";

function manhattan(s) {
  let d = 0;
  for (let i = 0; i < 9; i++) {
    if (s[i] === "0") continue;
    const v = s.charCodeAt(i) - 49; // '1' -> 0
    d += Math.abs(Math.floor(i / 3) - Math.floor(v / 3)) + Math.abs((i % 3) - (v % 3));
  }
  return d;
}

function solve(start) {
  const open = [[manhattan(start), 0, start]];
  const g = new Map([[start, 0]]);
  const parent = new Map();
  const dr = [-1, 1, 0, 0], dc = [0, 0, -1, 1];
  while (open.length) {
    // extract min by f (linear scan; search space is small)
    let mi = 0;
    for (let i = 1; i < open.length; i++) if (open[i][0] < open[mi][0]) mi = i;
    const [f, gc, cur] = open.splice(mi, 1)[0];
    if (cur === GOAL) break;
    if (gc > g.get(cur)) continue;
    const z = cur.indexOf("0"), r = Math.floor(z / 3), c = z % 3;
    for (let k = 0; k < 4; k++) {
      const nr = r + dr[k], nc = c + dc[k];
      if (nr < 0 || nr > 2 || nc < 0 || nc > 2) continue;
      const nz = nr * 3 + nc;
      const arr = cur.split("");
      [arr[z], arr[nz]] = [arr[nz], arr[z]];
      const nxt = arr.join("");
      const ng = gc + 1;
      if (!g.has(nxt) || ng < g.get(nxt)) {
        g.set(nxt, ng);
        parent.set(nxt, [cur, Number(cur[nz])]);
        open.push([ng + manhattan(nxt), ng, nxt]);
      }
    }
  }
  const moves = [];
  let cur = GOAL;
  while (cur !== start) {
    const [prev, tile] = parent.get(cur);
    moves.push(tile);
    cur = prev;
  }
  return moves.reverse();
}

const start = "123406758"; // [[1,2,3],[4,_,6],[7,5,8]]
const moves = solve(start);
console.log(`Solved in ${moves.length} moves: [${moves.join(", ")}]`);
