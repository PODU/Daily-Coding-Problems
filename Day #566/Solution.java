// Minimally-connected = tree: Union-Find detects cycle on union; final check connected && edges==V-1. Time O(E a(V)), Space O(V).
import java.util.List;

public class Solution {
    static int[] parent;

    static int find(int x) {
        while (parent[x] != x) {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    }

    static boolean isMinimallyConnected(int V, int[][] edges) {
        if (edges.length != V - 1) return false;
        parent = new int[V];
        for (int i = 0; i < V; i++) parent[i] = i;
        for (int[] e : edges) {
            int ra = find(e[0]), rb = find(e[1]);
            if (ra == rb) return false; // cycle
            parent[ra] = rb;
        }
        int root = find(0);
        for (int i = 1; i < V; i++)
            if (find(i) != root) return false; // not connected
        return true;
    }

    public static void main(String[] args) {
        System.out.println(isMinimallyConnected(4, new int[][]{{0,1},{1,2},{2,3}}) ? "True" : "False");
        System.out.println(isMinimallyConnected(4, new int[][]{{0,1},{1,2},{2,3},{3,0}}) ? "True" : "False");
    }
}
