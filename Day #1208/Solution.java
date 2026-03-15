// Day 1208: Fewest perfect squares summing to N.
// Lagrange four-square + Legendre's three-square theorem. Time O(sqrt N), Space O(1).
public class Solution {
    static boolean isSquare(long n) { long r = (long) Math.sqrt(n); while (r * r < n) r++; while (r * r > n) r--; return r * r == n; }

    static int numSquares(long n) {
        if (isSquare(n)) return 1;
        for (long a = 1; a * a <= n; a++) if (isSquare(n - a * a)) return 2;
        long m = n;
        while (m % 4 == 0) m /= 4;
        if (m % 8 == 7) return 4;
        return 3;
    }

    public static void main(String[] args) {
        System.out.println(numSquares(17)); // 2
    }
}
