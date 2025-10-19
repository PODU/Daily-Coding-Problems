// Day 459: Fewest perfect squares summing to N.
// Lagrange/Legendre theorem -> answer in {1,2,3,4}, O(sqrt N).
// Reconstruct one decomposition guided by the count.
import java.util.ArrayList;
import java.util.List;
import java.util.StringJoiner;

public class Solution {
    static boolean isSquare(long n) {
        if (n < 0) return false;
        long r = (long) Math.sqrt((double) n);
        while (r * r < n) r++;
        while (r * r > n) r--;
        return r * r == n;
    }

    static int minSquares(long n) {
        if (isSquare(n)) return 1;
        long m = n;
        while (m % 4 == 0) m /= 4;
        if (m % 8 == 7) return 4;
        for (long i = 1; i * i <= n; i++)
            if (isSquare(n - i * i)) return 2;
        return 3;
    }

    static List<Long> decompose(long n) {
        int k = minSquares(n);
        List<Long> res = new ArrayList<>();
        while (k > 0) {
            if (k == 1) { res.add(n); break; }
            for (long i = (long) Math.sqrt((double) n); i >= 1; i--) {
                if (minSquares(n - i * i) == k - 1) {
                    res.add(i * i);
                    n -= i * i;
                    k--;
                    break;
                }
            }
        }
        return res;
    }

    public static void main(String[] args) {
        long n = 17;
        StringJoiner sj = new StringJoiner(" + ");
        for (long s : decompose(n)) sj.add(String.valueOf(s));
        System.out.println(minSquares(n) + " (" + sj + ")"); // 2 (16 + 1)
    }
}
