// Day 670: Fewest perfect squares summing to n. Lagrange (answer in {1,2,3,4}) +
// Legendre three-square theorem. Time O(sqrt n), Space O(1).
public class Solution {
    static boolean isSquare(long n) { long r = (long) Math.sqrt(n); return r * r == n || (r + 1) * (r + 1) == n; }

    static int numSquares(long n) {
        if (isSquare(n)) return 1;
        for (long a = 1; a * a <= n; a++) if (isSquare(n - a * a)) return 2;
        long m = n;
        while (m % 4 == 0) m /= 4;
        if (m % 8 == 7) return 4;
        return 3;
    }

    public static void main(String[] args) {
        System.out.println(numSquares(13)); // 2
        System.out.println(numSquares(27)); // 3
    }
}
