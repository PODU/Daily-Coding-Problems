// Day 182: Graph is minimally-connected iff it is a tree (connected and |E| == |V|-1).
// BFS connectivity + edge count. Time O(V+E), Space O(V+E).
import java.util.*;

public class Solution {
    static boolean isMinimallyConnected(int v, int[][] edges) {
        if (edges.length != v - 1) return false;
        List<List<Integer>> adj = new ArrayList<>();
        for (int i = 0; i < v; i++) adj.add(new ArrayList<>());
        for (int[] e : edges) { adj.get(e[0]).add(e[1]); adj.get(e[1]).add(e[0]); }
        boolean[] seen = new boolean[v];
        Deque<Integer> q = new ArrayDeque<>();
        q.add(0); seen[0] = true; int cnt = 1;
        while (!q.isEmpty()) {
            int u = q.poll();
            for (int w : adj.get(u)) if (!seen[w]) { seen[w] = true; cnt++; q.add(w); }
        }
        return cnt == v;
    }

    public static void main(String[] args) {
        int[][] tree = {{0,1},{0,2},{1,3},{1,4}};
        int[][] cyclic = {{0,1},{0,2},{1,3},{1,4},{3,4}};
        System.out.println(isMinimallyConnected(5, tree));
        System.out.println(isMinimallyConnected(5, cyclic));
    }
}
