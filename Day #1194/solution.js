// 2-SAT via implication graph + Tarjan SCC. Node 2*v=(v true), 2*v+1=(v false).
// Clause (x OR y): add edges ~x->y and ~y->x. UNSAT iff x and ~x share an SCC.
// Pick literal whose SCC is the sink; verify against clauses. Time O(V+C).

class TwoSAT {
    constructor(n) {
        this.n = n;
        this.adj = Array.from({ length: 2 * n }, () => []);
        this.clauses = [];
        this.comp = new Array(2 * n).fill(-1);
        this.low = new Array(2 * n).fill(0);
        this.num = new Array(2 * n).fill(-1);
        this.onStk = new Array(2 * n).fill(false);
        this.stk = [];
        this.idx = 0;
        this.scc = 0;
    }

    static lit(v, neg) { return 2 * v + (neg ? 1 : 0); }

    clause(v1, n1, v2, n2) {
        this.adj[TwoSAT.lit(v1, !n1)].push(TwoSAT.lit(v2, n2));
        this.adj[TwoSAT.lit(v2, !n2)].push(TwoSAT.lit(v1, n1));
        this.clauses.push([v1, n1, v2, n2]);
    }

    tarjan(u) {
        this.low[u] = this.num[u] = this.idx++;
        this.stk.push(u); this.onStk[u] = true;
        for (const w of this.adj[u]) {
            if (this.num[w] === -1) { this.tarjan(w); this.low[u] = Math.min(this.low[u], this.low[w]); }
            else if (this.onStk[w]) this.low[u] = Math.min(this.low[u], this.num[w]);
        }
        if (this.low[u] === this.num[u]) {
            while (true) {
                const x = this.stk.pop(); this.onStk[x] = false;
                this.comp[x] = this.scc;
                if (x === u) break;
            }
            this.scc++;
        }
    }

    satisfies(val) {
        for (const [v1, n1, v2, n2] of this.clauses) {
            const a = val[v1] !== n1;
            const b = val[v2] !== n2;
            if (!(a || b)) return false;
        }
        return true;
    }

    solve() {
        for (let i = 0; i < 2 * this.n; i++)
            if (this.num[i] === -1) this.tarjan(i);
        for (let v = 0; v < this.n; v++)
            if (this.comp[TwoSAT.lit(v, false)] === this.comp[TwoSAT.lit(v, true)]) return null;
        let val = [];
        for (let v = 0; v < this.n; v++)
            val.push(this.comp[TwoSAT.lit(v, false)] < this.comp[TwoSAT.lit(v, true)]);
        if (!this.satisfies(val)) {
            val = [];
            for (let v = 0; v < this.n; v++)
                val.push(this.comp[TwoSAT.lit(v, true)] < this.comp[TwoSAT.lit(v, false)]);
        }
        return val;
    }
}

function main() {
    const ts = new TwoSAT(3); // a=0, b=1, c=2
    // (~c OR b), (b OR c), (~b OR c), (~c OR ~a)
    ts.clause(2, true, 1, false);
    ts.clause(1, false, 2, false);
    ts.clause(1, true, 2, false);
    ts.clause(2, true, 0, true);

    const val = ts.solve();
    if (val === null) { console.log("UNSATISFIABLE"); return; }
    const s = b => (b ? "True" : "False");
    console.log(`a = ${s(val[0])}, b = ${s(val[1])}, c = ${s(val[2])}`);
}

main();
