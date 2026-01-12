// Egyptian fraction via greedy: take ceil(b/a) each step. Time O(terms), Space O(1).
public class Solution {
    static String egyptian(long a, long b) {
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
        return sb.toString();
    }

    public static void main(String[] args) {
        System.out.println(egyptian(4, 13));
    }
}
