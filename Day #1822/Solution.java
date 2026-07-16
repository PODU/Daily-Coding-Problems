// Bipartite check via BFS 2-coloring. O(V+E) time, O(V) space.
import java.util.*;

public class Solution {
    static boolean isBipartite(int n, List<List<Integer>> adj){
        int[] color = new int[n];
        Arrays.fill(color, -1);
        for(int s = 0; s < n; s++){
            if(color[s] != -1) continue;
            Deque<Integer> q = new ArrayDeque<>();
            q.add(s); color[s] = 0;
            while(!q.isEmpty()){
                int u = q.poll();
                for(int v : adj.get(u)){
                    if(color[v] == -1){ color[v] = color[u] ^ 1; q.add(v); }
                    else if(color[v] == color[u]) return false;
                }
            }
        }
        return true;
    }
    public static void main(String[] args){
        int n = 4;
        List<List<Integer>> adj = new ArrayList<>();
        for(int i = 0; i < n; i++) adj.add(new ArrayList<>());
        int[][] edges = {{0,1},{1,2},{2,3},{3,0}}; // even cycle -> bipartite
        for(int[] e : edges){ adj.get(e[0]).add(e[1]); adj.get(e[1]).add(e[0]); }
        System.out.println(isBipartite(n, adj) ? "true" : "false");
    }
}
