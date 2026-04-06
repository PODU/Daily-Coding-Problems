// Egyptian fraction via Fibonacci/Sylvester greedy: repeatedly subtract the
// largest unit fraction 1/ceil(b/a). Time O(#terms), Space O(1).
public class Solution {
    static String egyptian(long a, long b) {
        StringBuilder sb = new StringBuilder();
        boolean first = true;
        while (a != 0) {
            long x = (b + a - 1) / a; // ceil(b/a)
            if (!first) sb.append(" + ");
            sb.append("1 / ").append(x);
            a = a * x - b;
            b = b * x;
            first = false;
        }
        return sb.toString();
    }

    public static void main(String[] args) {
        System.out.println(egyptian(4, 13)); // 1 / 4 + 1 / 18 + 1 / 468
    }
}
