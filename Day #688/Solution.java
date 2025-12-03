// 2-SAT: implication graph + iterative Tarjan SCC. Sat iff no var shares SCC with its negation.
// Time O(n + m), Space O(n + m).
import java.util.*;

public class Solution {
    static int n;
    static List<List<Integer>> g;
    static int[] idx, low, comp;
    static boolean[] onstk;
    static Deque<Integer> stk = new ArrayDeque<>();
    static int cnt = 0, cc = 0;

    static int node(int var, boolean sign) { return 2 * var + (sign ? 0 : 1); }
    static int neg(int x) { return x ^ 1; }

    static void addClause(int v1, boolean s1, int v2, boolean s2) {
        int x = node(v1, s1), y = node(v2, s2);
        g.get(neg(x)).add(y);
        g.get(neg(y)).add(x);
    }

    static void tarjan(int start) {
        Deque<int[]> work = new ArrayDeque<>();
        work.push(new int[]{start, 0});
        while (!work.isEmpty()) {
            int[] top = work.peek();
            int v = top[0], pi = top[1];
            if (pi == 0) {
                idx[v] = low[v] = cnt++;
                stk.push(v);
                onstk[v] = true;
            }
            boolean recurse = false;
            int i = pi;
            for (; i < g.get(v).size(); i++) {
                int w = g.get(v).get(i);
                if (idx[w] == -1) {
                    top[1] = i + 1;
                    work.push(new int[]{w, 0});
                    recurse = true;
                    break;
                } else if (onstk[w]) {
                    low[v] = Math.min(low[v], idx[w]);
                }
            }
            if (recurse) continue;
            if (low[v] == idx[v]) {
                while (true) {
                    int w = stk.pop();
                    onstk[w] = false;
                    comp[w] = cc;
                    if (w == v) break;
                }
                cc++;
            }
            work.pop();
            if (!work.isEmpty()) {
                int pv = work.peek()[0];
                low[pv] = Math.min(low[pv], low[v]);
            }
        }
    }

    static boolean solve(boolean[] assign) {
        for (int i = 0; i < 2 * n; i++)
            if (idx[i] == -1) tarjan(i);
        for (int v = 0; v < n; v++) {
            if (comp[2 * v] == comp[2 * v + 1]) return false;
            assign[v] = comp[2 * v] < comp[2 * v + 1];
        }
        return true;
    }

    public static void main(String[] args) {
        // Variables a=0, b=1, c=2. sign true=positive, false=negated.
        // (!c OR b) AND (b OR c) AND (!b OR c) AND (!c OR !a)
        n = 3;
        g = new ArrayList<>();
        for (int i = 0; i < 2 * n; i++) g.add(new ArrayList<>());
        idx = new int[2 * n];
        low = new int[2 * n];
        comp = new int[2 * n];
        onstk = new boolean[2 * n];
        Arrays.fill(idx, -1);
        Arrays.fill(comp, -1);

        addClause(2, false, 1, true);
        addClause(1, true, 2, true);
        addClause(1, false, 2, true);
        addClause(2, false, 0, false);

        boolean[] assign = new boolean[n];
        if (!solve(assign)) {
            System.out.println("UNSATISFIABLE");
            return;
        }
        String[] names = {"a", "b", "c"};
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < n; i++) {
            if (i > 0) sb.append(", ");
            sb.append(names[i]).append(" = ").append(assign[i] ? "True" : "False");
        }
        System.out.println(sb.toString());
    }
}
