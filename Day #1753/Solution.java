// Day 1753: Egyptian fraction (sum of distinct unit fractions).
// Greedy Fibonacci-Sylvester: repeatedly subtract 1/ceil(b/a). Time O(terms), O(1) space.
public class Solution {
    static String egyptian(long a, long b) {
        StringBuilder sb = new StringBuilder();
        boolean first = true;
        while (a != 0) {
            long c = (b + a - 1) / a; // ceil(b/a)
            if (!first) sb.append(" + ");
            sb.append("1 / ").append(c);
            first = false;
            a = a * c - b;
            b = b * c;
            long g = gcd(Math.abs(a), b);
            if (g > 1) { a /= g; b /= g; }
        }
        return sb.toString();
    }

    static long gcd(long x, long y) {
        while (y != 0) { long t = x % y; x = y; y = t; }
        return x;
    }

    public static void main(String[] args) {
        // README example: 4 / 13 -> 1 / 4 + 1 / 18 + 1 / 468
        System.out.println(egyptian(4, 13));
    }
}
