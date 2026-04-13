// Count max-heaps: count(n)=C(n-1,L)*count(L)*count(R), L=left subtree size from heap shape.
// Time: O(N^2) (binomial table + recursion), Space: O(N^2).
public class Solution {
    static long[][] C;

    static int leftSize(int n) {
        if (n == 1) return 0;
        int h = (int) Math.floor(Math.log(n) / Math.log(2));
        int lower = 1 << (h - 1);
        int last = n - ((1 << h) - 1);
        int leftLast = Math.min(last, lower);
        return ((1 << (h - 1)) - 1) + leftLast;
    }

    static long countHeaps(int n) {
        if (n <= 1) return 1;
        int L = leftSize(n);
        int R = n - 1 - L;
        return C[n - 1][L] * countHeaps(L) * countHeaps(R);
    }

    public static void main(String[] args) {
        int N = 3;
        C = new long[N + 1][N + 1];
        for (int i = 0; i <= N; i++) {
            C[i][0] = 1;
            for (int j = 1; j <= i; j++)
                C[i][j] = C[i - 1][j - 1] + (j <= i - 1 ? C[i - 1][j] : 0);
        }
        System.out.println(countHeaps(N));
    }
}
