// add_subtract: chainable builder. add(x) alternates +/- ; value() reads result.
// O(n) time over number of args, O(1) extra space.
public class Solution {
    static class AddSub {
        final long total;
        final int sign; // applied to next argument
        AddSub(long total, int sign) { this.total = total; this.sign = sign; }
        AddSub add(long x) { return new AddSub(total + sign * x, -sign); }
        long value() { return total; }
    }

    static AddSub add_subtract(long first) { return new AddSub(first, 1); }

    public static void main(String[] args) {
        System.out.println(add_subtract(7).value());                 // 7
        System.out.println(add_subtract(1).add(2).add(3).value());   // 0
        System.out.println(add_subtract(-5).add(10).add(3).add(9).value()); // 11
    }
}
