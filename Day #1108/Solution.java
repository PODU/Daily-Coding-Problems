// Day 1108: Detect a cycle in an undirected graph using Union-Find.
// If an edge connects two already-connected vertices, there's a cycle.
// Time: O(E * alpha(V)). Space: O(V).
public class Solution {
    static int[] p, r;
    static int find(int x){ return p[x]==x ? x : (p[x]=find(p[x])); }
    static boolean unite(int a, int b){
        a=find(a); b=find(b);
        if (a==b) return false;
        if (r[a]<r[b]){ int t=a; a=b; b=t; }
        p[b]=a; if (r[a]==r[b]) r[a]++;
        return true;
    }
    static boolean hasCycle(int n, int[][] edges){
        p = new int[n]; r = new int[n];
        for (int i=0;i<n;i++) p[i]=i;
        for (int[] e : edges) if (!unite(e[0], e[1])) return true;
        return false;
    }
    public static void main(String[] args){
        System.out.println(hasCycle(3, new int[][]{{0,1},{1,2},{2,0}})); // true
        System.out.println(hasCycle(4, new int[][]{{0,1},{1,2},{2,3}})); // false
    }
}
