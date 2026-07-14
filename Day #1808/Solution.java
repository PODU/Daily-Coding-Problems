// Curried add_subtract: each call alternates +/- on the running total.
// Java has no call operator, so we expose a fluent .apply(...) returning a chainable value.
// Time: O(1) per call. Space: O(1).
public class Solution {
    static final class AddSub {
        final long total;
        final int sign; // sign applied to the NEXT argument
        AddSub(long total, int sign) { this.total = total; this.sign = sign; }
        AddSub apply(long y) { return new AddSub(total + sign * y, -sign); }
        long value() { return total; }
        public String toString() { return Long.toString(total); }
    }

    static AddSub add_subtract(long x) { return new AddSub(x, 1); }

    public static void main(String[] args) {
        System.out.println(add_subtract(7));                            // 7
        System.out.println(add_subtract(1).apply(2).apply(3));          // 0
        System.out.println(add_subtract(-5).apply(10).apply(3).apply(9)); // 11
    }
}
