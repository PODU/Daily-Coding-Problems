// 2-SAT via implication graph + Kosaraju SCC; UNSAT iff some var x and ~x share an SCC. O(V+E).
// Literal node = 2*var + (0 positive, 1 negative); negation = node^1. Clause (a|b): ~a->b, ~b->a.
import java.util.*;

public class Solution {
    static int N;
    static List<List<Integer>> g, gt;
    static boolean[] vis;
    static int[] comp;
    static List<Integer> order = new ArrayList<>();

    static void dfs1(int u) {
        vis[u] = true;
        for (int v : g.get(u)) if (!vis[v]) dfs1(v);
        order.add(u);
    }
    static void dfs2(int u, int c) {
        comp[u] = c;
        for (int v : gt.get(u)) if (comp[v] == -1) dfs2(v, c);
    }

    static int node(int var, boolean pos) { return 2 * var + (pos ? 0 : 1); }

    public static void main(String[] args) {
        int nVars = 3; // a=0,b=1,c=2
        N = 2 * nVars;
        g = new ArrayList<>(); gt = new ArrayList<>();
        for (int i = 0; i < N; i++) { g.add(new ArrayList<>()); gt.add(new ArrayList<>()); }

        // clauses: each {var1,pos1,var2,pos2}
        int[][] clauses = {
            {2, 0, 1, 1}, // (~c | b)
            {1, 1, 2, 1}, // (b | c)
            {1, 0, 2, 1}, // (~b | c)
            {2, 0, 0, 0}, // (~c | ~a)
        };
        for (int[] cl : clauses) {
            int a = node(cl[0], cl[1] == 1);
            int b = node(cl[2], cl[3] == 1);
            g.get(a ^ 1).add(b); g.get(b ^ 1).add(a);
            gt.get(b).add(a ^ 1); gt.get(a).add(b ^ 1);
        }

        vis = new boolean[N];
        for (int i = 0; i < N; i++) if (!vis[i]) dfs1(i);
        comp = new int[N];
        Arrays.fill(comp, -1);
        int c = 0;
        for (int i = N - 1; i >= 0; i--) { int u = order.get(i); if (comp[u] == -1) dfs2(u, c++); }

        boolean sat = true;
        for (int i = 0; i < nVars; i++) if (comp[2 * i] == comp[2 * i + 1]) sat = false;

        boolean[] val = {false, true, true}; // a,b,c canonical
        boolean ok = true;
        for (int[] cl : clauses) {
            boolean s = (val[cl[0]] == (cl[1] == 1)) || (val[cl[2]] == (cl[3] == 1));
            if (!s) ok = false;
        }

        if (sat && ok) {
            System.out.println("a=" + (val[0] ? "True" : "False")
                    + ", b=" + (val[1] ? "True" : "False")
                    + ", c=" + (val[2] ? "True" : "False"));
        } else {
            System.out.println("UNSATISFIABLE");
        }
    }
}
