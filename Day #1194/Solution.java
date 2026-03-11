// 2-SAT via implication graph + Tarjan SCC. Node 2*v=(v true), 2*v+1=(v false).
// Clause (x OR y): add edges ~x->y and ~y->x. UNSAT iff x and ~x share an SCC.
// Pick literal whose SCC is the sink; verify against clauses. Time O(V+C).
import java.util.*;

public class Solution {
    static int V;
    static List<List<Integer>> adj;
    static int[] comp, low, num;
    static boolean[] onStk;
    static Deque<Integer> stk = new ArrayDeque<>();
    static int idx = 0, sccCount = 0;
    static List<int[]> clauses = new ArrayList<>();

    static int lit(int v, boolean neg) { return 2 * v + (neg ? 1 : 0); }

    static void tarjan(int u) {
        low[u] = num[u] = idx++;
        stk.push(u); onStk[u] = true;
        for (int w : adj.get(u)) {
            if (num[w] == -1) { tarjan(w); low[u] = Math.min(low[u], low[w]); }
            else if (onStk[w]) low[u] = Math.min(low[u], num[w]);
        }
        if (low[u] == num[u]) {
            while (true) {
                int x = stk.pop(); onStk[x] = false;
                comp[x] = sccCount;
                if (x == u) break;
            }
            sccCount++;
        }
    }

    static void clause(int v1, boolean n1, int v2, boolean n2) {
        adj.get(lit(v1, !n1)).add(lit(v2, n2));
        adj.get(lit(v2, !n2)).add(lit(v1, n1));
        clauses.add(new int[]{v1, n1 ? 1 : 0, v2, n2 ? 1 : 0});
    }

    static boolean satisfies(boolean[] val) {
        for (int[] c : clauses) {
            boolean a = val[c[0]] != (c[1] == 1);
            boolean b = val[c[2]] != (c[3] == 1);
            if (!(a || b)) return false;
        }
        return true;
    }

    public static void main(String[] args) {
        V = 3; // a=0, b=1, c=2
        adj = new ArrayList<>();
        for (int i = 0; i < 2 * V; i++) adj.add(new ArrayList<>());
        comp = new int[2 * V]; low = new int[2 * V];
        num = new int[2 * V]; onStk = new boolean[2 * V];
        Arrays.fill(comp, -1); Arrays.fill(num, -1);

        // (~c OR b), (b OR c), (~b OR c), (~c OR ~a)
        clause(2, true, 1, false);
        clause(1, false, 2, false);
        clause(1, true, 2, false);
        clause(2, true, 0, true);

        for (int i = 0; i < 2 * V; i++)
            if (num[i] == -1) tarjan(i);

        for (int v = 0; v < V; v++)
            if (comp[lit(v, false)] == comp[lit(v, true)]) {
                System.out.println("UNSATISFIABLE");
                return;
            }

        boolean[] val = new boolean[V];
        for (int v = 0; v < V; v++) val[v] = comp[lit(v, false)] < comp[lit(v, true)];
        if (!satisfies(val))
            for (int v = 0; v < V; v++) val[v] = comp[lit(v, true)] < comp[lit(v, false)];

        System.out.println("a = " + (val[0] ? "True" : "False")
                + ", b = " + (val[1] ? "True" : "False")
                + ", c = " + (val[2] ? "True" : "False"));
    }
}
