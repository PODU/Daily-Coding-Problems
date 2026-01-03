// Day 844: 2-SAT via implication graph + Kosaraju SCC.
// Clause (x OR y) adds !x->y and !y->x. SAT iff no variable shares an SCC with its negation. O(V+E).
import java.util.*;

public class Solution {
    static int n;
    static List<List<Integer>> adj, radj;
    static boolean[] vis;
    static int[] comp;
    static List<Integer> order;

    static int node(int var, boolean neg){ return 2*var + (neg ? 1 : 0); }

    static void addClause(int va, boolean na, int vb, boolean nb){
        int a = node(va, na), b = node(vb, nb);
        adj.get(a^1).add(b);
        adj.get(b^1).add(a);
    }

    static void dfs1(int u){
        vis[u] = true;
        for(int v : adj.get(u)) if(!vis[v]) dfs1(v);
        order.add(u);
    }
    static void dfs2(int u, int c){
        comp[u] = c;
        for(int v : radj.get(u)) if(comp[v] == -1) dfs2(v, c);
    }

    static boolean[] solve(){
        vis = new boolean[2*n];
        order = new ArrayList<>();
        for(int i = 0; i < 2*n; i++) if(!vis[i]) dfs1(i);
        radj = new ArrayList<>();
        for(int i = 0; i < 2*n; i++) radj.add(new ArrayList<>());
        for(int u = 0; u < 2*n; u++) for(int v : adj.get(u)) radj.get(v).add(u);
        comp = new int[2*n];
        Arrays.fill(comp, -1);
        int c = 0;
        for(int i = 2*n-1; i >= 0; i--){ int u = order.get(i); if(comp[u] == -1) dfs2(u, c++); }
        boolean[] assign = new boolean[n];
        for(int v = 0; v < n; v++){
            if(comp[2*v] == comp[2*v+1]) return null;
            assign[v] = comp[2*v] > comp[2*v+1];
        }
        return assign;
    }

    public static void main(String[] args){
        n = 3; // a=0, b=1, c=2
        adj = new ArrayList<>();
        for(int i = 0; i < 2*n; i++) adj.add(new ArrayList<>());
        addClause(2,true, 1,false); // ¬c OR b
        addClause(1,false,2,false); // b OR c
        addClause(1,true, 2,false); // ¬b OR c
        addClause(2,true, 0,true);  // ¬c OR ¬a
        boolean[] a = solve();
        if(a == null) System.out.println("False");
        else System.out.println("a = " + (a[0]?"True":"False")
            + ", b = " + (a[1]?"True":"False")
            + ", c = " + (a[2]?"True":"False"));
    }
}
