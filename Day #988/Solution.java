// Day 988: Minimum number of perfect squares summing to n.
// Number theory (Lagrange + Legendre's three-square theorem). O(sqrt n) time, O(1) space.
public class Solution {
    static boolean isSquare(long x) {
        long r = (long) Math.sqrt((double) x);
        while (r * r > x) r--;
        while ((r + 1) * (r + 1) <= x) r++;
        return r * r == x;
    }

    static int numSquares(long n) {
        if (isSquare(n)) return 1;
        long m = n;
        while (m % 4 == 0) m /= 4;        // strip factors of 4
        if (m % 8 == 7) return 4;         // Legendre: 4^a(8b+7) needs 4
        for (long a = 1; a * a <= n; a++)
            if (isSquare(n - a * a)) return 2;
        return 3;
    }

    public static void main(String[] args) {
        System.out.println("13 -> " + numSquares(13)); // expected 2
        System.out.println("27 -> " + numSquares(27)); // expected 3
    }
}
