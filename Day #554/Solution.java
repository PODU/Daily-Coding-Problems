// Egyptian fraction via greedy (Fibonacci/Sylvester): take ceil(b/a) each step.
// Time: O(number of terms), Space: O(1). 64-bit longs.
public class Solution {
    public static void main(String[] args) {
        long a = 4, b = 13;
        StringBuilder sb = new StringBuilder();
        boolean first = true;
        while (a != 0) {
            long x = (b + a - 1) / a; // ceil(b/a)
            if (!first) sb.append(" + ");
            sb.append("1 / ").append(x);
            first = false;
            a = a * x - b;
            b = b * x;
        }
        System.out.println(sb.toString());
    }
}
