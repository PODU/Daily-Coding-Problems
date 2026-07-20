// Day 1847: 2-SAT solver. Build implication graph, find SCCs (Kosaraju), check x and ¬x differ.
// Time O(V + C), Space O(V + C). Literal node = 2*var + (negated?1:0); neg(x)=x^1.
import java.util.*;

public class Solution {
    static int n;
    static List<List<Integer>> adj, adjT;
    static boolean[] used;
    static int[] comp;
    static List<Integer> order;

    static void addOr(int u, int v) {
        adj.get(u ^ 1).add(v); adjT.get(v).add(u ^ 1);
        adj.get(v ^ 1).add(u); adjT.get(u).add(v ^ 1);
    }
    static void dfs1(int v) {
        used[v] = true;
        for (int to : adj.get(v)) if (!used[to]) dfs1(to);
        order.add(v);
    }
    static void dfs2(int v, int c) {
        comp[v] = c;
        for (int to : adjT.get(v)) if (comp[to] == -1) dfs2(to, c);
    }

    static boolean[] solve() {
        used = new boolean[2 * n]; order = new ArrayList<>();
        for (int i = 0; i < 2 * n; i++) if (!used[i]) dfs1(i);
        comp = new int[2 * n]; Arrays.fill(comp, -1);
        for (int i = 0, c = 0; i < 2 * n; i++) {
            int v = order.get(2 * n - 1 - i);
            if (comp[v] == -1) dfs2(v, c++);
        }
        boolean[] res = new boolean[n];
        for (int i = 0; i < n; i++) {
            if (comp[2 * i] == comp[2 * i + 1]) return null;
            res[i] = comp[2 * i] > comp[2 * i + 1];
        }
        return res;
    }

    static int lit(int v, int neg) { return 2 * v + neg; }

    public static void main(String[] args) {
        n = 3;
        adj = new ArrayList<>(); adjT = new ArrayList<>();
        for (int i = 0; i < 2 * n; i++) { adj.add(new ArrayList<>()); adjT.add(new ArrayList<>()); }
        addOr(lit(2, 1), lit(1, 0)); // (¬c OR b)
        addOr(lit(1, 0), lit(2, 0)); // (b OR c)
        addOr(lit(1, 1), lit(2, 0)); // (¬b OR c)
        addOr(lit(2, 1), lit(0, 1)); // (¬c OR ¬a)

        boolean[] a = solve();
        if (a == null) { System.out.println("False"); return; }
        String names = "abc";
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < n; i++) {
            sb.append(names.charAt(i)).append(" = ").append(a[i] ? "True" : "False");
            if (i < n - 1) sb.append(", ");
        }
        System.out.println(sb);
    }
}
