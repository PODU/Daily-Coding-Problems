// Couples holding hands: union the two couples occupying each seat-pair. Minimum swaps =
// N - (number of connected components among couples). Time: O(N alpha), Space: O(N).
public class Solution {
    static int[] parent;
    static int comps;

    static int find(int x) {
        while (parent[x] != x) { parent[x] = parent[parent[x]]; x = parent[x]; }
        return x;
    }

    static void unite(int a, int b) {
        a = find(a); b = find(b);
        if (a != b) { parent[a] = b; comps--; }
    }

    static int minSwaps(int[] row) {
        int n = row.length / 2;
        parent = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
        comps = n;
        for (int i = 0; i < row.length; i += 2)
            unite(row[i] / 2, row[i + 1] / 2);
        return n - comps;
    }

    public static void main(String[] args) {
        int[] row = {0, 2, 1, 3};
        System.out.println(minSwaps(row)); // 1
    }
}
