// Count distinct max-heaps from N distinct values: ways(n)=C(n-1,L)*ways(L)*ways(R).
// L = size of left subtree of a complete binary tree with n nodes. Time: O(n^2). Space: O(n^2).
public class Solution {
    static int leftSubtreeSize(int n) {
        int h = (int) Math.floor(Math.log(n) / Math.log(2));
        int m = n - ((1 << h) - 1);
        int lastCap = 1 << (h - 1);
        return ((1 << (h - 1)) - 1) + Math.min(m, lastCap);
    }

    public static void main(String[] args) {
        int N = 3;
        long[] ways = new long[N + 1];
        long[][] C = new long[N + 1][N + 1];
        for (int i = 0; i <= N; i++) {
            C[i][0] = 1;
            for (int j = 1; j <= i; j++) C[i][j] = C[i-1][j-1] + C[i-1][j];
        }
        ways[0] = 1;
        if (N >= 1) ways[1] = 1;
        for (int n = 2; n <= N; n++) {
            int L = leftSubtreeSize(n);
            int R = n - 1 - L;
            ways[n] = C[n-1][L] * ways[L] * ways[R];
        }
        System.out.println(ways[N]);
    }
}
