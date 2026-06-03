// Count distinct max-heaps from N distinct integers. ways(n)=C(n-1,L)*ways(L)*ways(R),
// L = left-subtree node count of a complete binary tree of n nodes. Time O(n^2), Space O(n^2).
public class Solution {
    static long[][] C = new long[64][64];

    static int leftCount(int n) {
        if (n == 1) return 0;
        int h = (int) Math.floor(Math.log(n) / Math.log(2)); // height (root at level 0)
        int last = n - ((1 << h) - 1);                        // nodes in the bottom level
        int maxLast = 1 << (h - 1);                           // max bottom-level nodes for left subtree
        return ((1 << (h - 1)) - 1) + Math.min(last, maxLast);
    }

    static long ways(int n) {
        if (n <= 1) return 1;
        int L = leftCount(n), R = n - 1 - L;
        return C[n - 1][L] * ways(L) * ways(R);
    }

    public static void main(String[] args) {
        for (int i = 0; i < 64; i++) {
            C[i][0] = 1;
            for (int j = 1; j <= i; j++) C[i][j] = C[i - 1][j - 1] + C[i - 1][j];
        }
        int[] arr = {1, 2, 3};   // N distinct integers
        System.out.println(ways(arr.length));
    }
}
