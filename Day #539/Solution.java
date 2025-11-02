// Day 539: Detect a cycle in an undirected graph using union-find (disjoint set).
// For each edge, if endpoints already share a root -> cycle. Time O(E*alpha(V)), Space O(V).
import java.util.*;

public class Solution {
    static int[] p, r;

    static int find(int x) { return p[x] == x ? x : (p[x] = find(p[x])); }

    static boolean unite(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return false;            // already connected -> cycle
        if (r[a] < r[b]) { int t = a; a = b; b = t; }
        p[b] = a;
        if (r[a] == r[b]) r[a]++;
        return true;
    }

    static boolean hasCycle(int n, int[][] edges) {
        p = new int[n]; r = new int[n];
        for (int i = 0; i < n; i++) p[i] = i;
        for (int[] e : edges)
            if (!unite(e[0], e[1])) return true;
        return false;
    }

    public static void main(String[] args) {
        int[][] cyclic = {{0,1},{1,2},{2,3},{3,0}};
        int[][] tree   = {{0,1},{1,2},{2,3}};
        System.out.println(hasCycle(4, cyclic));
        System.out.println(hasCycle(4, tree));
    }
}
