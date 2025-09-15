// Day 280: Detect cycle in undirected graph via Union-Find.
// An edge joining two already-connected vertices implies a cycle.
// Time O(V + E * alpha(V)), Space O(V).
public class Solution {
    static int[] p;

    static int find(int x) {
        while (p[x] != x) { p[x] = p[p[x]]; x = p[x]; }
        return x;
    }

    static boolean union(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return false;
        p[a] = b;
        return true;
    }

    static boolean hasCycle(int n, int[][] edges) {
        p = new int[n];
        for (int i = 0; i < n; i++) p[i] = i;
        for (int[] e : edges)
            if (!union(e[0], e[1])) return true;
        return false;
    }

    public static void main(String[] args) {
        int n = 4;
        System.out.println(hasCycle(n, new int[][]{{0,1},{1,2},{2,0},{2,3}})); // true
        System.out.println(hasCycle(n, new int[][]{{0,1},{1,2},{2,3}}));        // false
    }
}
