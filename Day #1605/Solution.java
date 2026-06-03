// 2xN tiling with dominoes + L-trominoes (LeetCode 790). DP recurrence
// f(n)=2*f(n-1)+f(n-3). O(N) time, O(1) space. Mod 1e9+7 for large N.
public class Solution {
    static final long MOD = 1_000_000_007L;

    static long numTilings(int n) {
        if (n == 0) return 1;
        if (n == 1) return 1;
        if (n == 2) return 2;
        long a = 1, b = 1, c = 2; // f(0),f(1),f(2)
        for (int i = 3; i <= n; i++) {
            long f = (2 * c + a) % MOD;
            a = b; b = c; c = f;
        }
        return c;
    }

    public static void main(String[] args) {
        System.out.println("N=4 -> " + numTilings(4)); // 11
        StringBuilder sb = new StringBuilder("Table N=1..5: ");
        for (int n = 1; n <= 5; n++) {
            sb.append(numTilings(n));
            if (n < 5) sb.append(" ");
        }
        System.out.println(sb); // 1 2 5 11 24
    }
}
