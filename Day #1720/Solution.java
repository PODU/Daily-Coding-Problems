// Min perfect squares: Legendre/Lagrange three-square theorem gives the count in
// O(sqrt N); decomposition found greedily largest-square-first. Time O(sqrt N), Space O(1).
import java.util.*;

public class Solution {
    static boolean isSquare(long n) {
        long r = (long) Math.sqrt((double) n);
        while (r * r > n) r--;
        while ((r + 1) * (r + 1) <= n) r++;
        return r * r == n;
    }

    static int minSquaresCount(long n) {
        if (isSquare(n)) return 1;
        for (long a = 1; a * a <= n; a++)
            if (isSquare(n - a * a)) return 2;
        long m = n;
        while (m % 4 == 0) m /= 4;
        if (m % 8 == 7) return 4;
        return 3;
    }

    static boolean find(long n, int count, List<Long> out) {
        if (count == 1) {
            if (isSquare(n)) { out.add(n); return true; }
            return false;
        }
        for (long a = (long) Math.sqrt((double) n) + 1; a >= 1; a--) {
            if (a * a > n) continue;
            if (find(n - a * a, count - 1, out)) { out.add(a * a); return true; }
        }
        return false;
    }

    static void demo(long n) {
        int count = minSquaresCount(n);
        List<Long> parts = new ArrayList<>();
        find(n, count, parts);
        parts.sort(Collections.reverseOrder());
        StringBuilder sb = new StringBuilder();
        sb.append(count).append(" (");
        for (int i = 0; i < parts.size(); i++) {
            if (i > 0) sb.append(" + ");
            sb.append(parts.get(i));
        }
        sb.append(")");
        System.out.println(sb);
    }

    public static void main(String[] args) {
        demo(4);
        demo(17);
        demo(18);
    }
}
