// Day 851: a graph is minimally-connected iff it is a tree: connected AND edges == nodes-1.
// Union-Find: detect cycle and connectivity in O(E alpha(V)).
public class Solution {
    static int[] p, r;
    static int find(int x){ return p[x] == x ? x : (p[x] = find(p[x])); }
    static boolean uni(int a, int b){
        a = find(a); b = find(b);
        if(a == b) return false;
        if(r[a] < r[b]){ int t = a; a = b; b = t; }
        p[b] = a; if(r[a] == r[b]) r[a]++;
        return true;
    }
    static boolean isMinimallyConnected(int n, int[][] edges){
        if(edges.length != n - 1) return false;
        p = new int[n]; r = new int[n];
        for(int i = 0; i < n; i++) p[i] = i;
        for(int[] e : edges) if(!uni(e[0], e[1])) return false; // cycle
        int comp = 0;
        for(int i = 0; i < n; i++) if(find(i) == i) comp++;
        return comp == 1;
    }
    public static void main(String[] args){
        System.out.println(isMinimallyConnected(5, new int[][]{{0,1},{0,2},{1,3},{1,4}}) ? "True" : "False"); // True
        System.out.println(isMinimallyConnected(4, new int[][]{{0,1},{1,2},{2,0},{2,3}}) ? "True" : "False"); // False
    }
}
