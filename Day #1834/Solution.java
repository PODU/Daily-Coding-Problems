// Min swaps to seat couples together. Union couples in each seat-pair; answer is
// N - (#connected components). O(N * alpha(N)).
public class Solution {
    static int[] par;
    static int find(int x){ return par[x]==x ? x : (par[x]=find(par[x])); }

    static int minSwaps(int[] row){
        int n = row.length / 2;
        par = new int[n];
        for(int i = 0; i < n; i++) par[i] = i;
        int comps = n;
        for(int i = 0; i < n; i++){
            int ra = find(row[2*i] / 2), rb = find(row[2*i+1] / 2);
            if(ra != rb){ par[ra] = rb; comps--; }
        }
        return n - comps;
    }
    public static void main(String[] args){
        int[] row = {0, 2, 1, 3}; // couples are (0,1) and (2,3)
        System.out.println(minSwaps(row)); // 1
    }
}
