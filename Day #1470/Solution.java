// Least squares summing to n via Legendre/Lagrange: 1 if square, 4 if 4^a(8b+7),
// 2 if i^2+j^2, else 3. Time: O(sqrt(n)); Space: O(1).
public class Solution {
    static boolean isSquare(long x) {
        long r = (long) Math.sqrt((double) x);
        while (r * r < x) r++;
        while (r * r > x) r--;
        return r * r == x;
    }

    static int numSquares(long n) {
        if (isSquare(n)) return 1;
        long m = n;
        while (m % 4 == 0) m /= 4;
        if (m % 8 == 7) return 4;
        for (long i = 1; i * i <= n; i++)
            if (isSquare(n - i * i)) return 2;
        return 3;
    }

    public static void main(String[] args) {
        System.out.println(numSquares(13));
        System.out.println(numSquares(27));
    }
}
