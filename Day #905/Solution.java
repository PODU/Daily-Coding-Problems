// Word-chain circle = Eulerian circuit on directed graph (node=letter, edge first->last).
// Check in==out degree for all nodes and single SCC over non-zero-degree nodes. O(V+E).
import java.util.*;

public class Solution {
    static int[] indeg = new int[26], outdeg = new int[26];
    static List<List<Integer>> adj = new ArrayList<>(), radj = new ArrayList<>();
    static boolean[] vis = new boolean[26];

    static void dfs(List<List<Integer>> g, int u) {
        vis[u] = true;
        for (int v : g.get(u)) if (!vis[v]) dfs(g, v);
    }

    static boolean canChain(String[] words) {
        adj.clear(); radj.clear();
        for (int i = 0; i < 26; i++) { indeg[i]=0; outdeg[i]=0; adj.add(new ArrayList<>()); radj.add(new ArrayList<>()); }
        for (String w : words) {
            int a = w.charAt(0)-'a', b = w.charAt(w.length()-1)-'a';
            outdeg[a]++; indeg[b]++;
            adj.get(a).add(b); radj.get(b).add(a);
        }
        for (int i = 0; i < 26; i++) if (indeg[i] != outdeg[i]) return false;
        int start = -1;
        for (int i = 0; i < 26; i++) if (outdeg[i] > 0) { start = i; break; }
        if (start == -1) return true;
        Arrays.fill(vis, false);
        dfs(adj, start);
        for (int i = 0; i < 26; i++) if ((indeg[i]!=0||outdeg[i]!=0) && !vis[i]) return false;
        Arrays.fill(vis, false);
        dfs(radj, start);
        for (int i = 0; i < 26; i++) if ((indeg[i]!=0||outdeg[i]!=0) && !vis[i]) return false;
        return true;
    }

    public static void main(String[] args) {
        String[] words = {"chair","height","racket","touch","tunic"};
        System.out.println(canChain(words) ? "True" : "False");
    }
}
