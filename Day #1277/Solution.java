// Day 1277: Curried add_subtract — alternately + then - successive args.
// Currying emulated via chained .apply(x); .value() reads the running result. O(1)/call.
public class Solution {
    static final class AddSubtract {
        final long value;
        final int idx; // args consumed so far
        AddSubtract(long value, int idx) { this.value = value; this.idx = idx; }
        AddSubtract apply(long x) {
            long nv = (idx % 2 == 1) ? value + x : value - x;
            return new AddSubtract(nv, idx + 1);
        }
        long value() { return value; }
    }

    static AddSubtract addSubtract(long x) { return new AddSubtract(x, 1); }

    public static void main(String[] args) {
        System.out.println(addSubtract(7).value());                       // 7
        System.out.println(addSubtract(1).apply(2).apply(3).value());      // 0
        System.out.println(addSubtract(-5).apply(10).apply(3).apply(9).value()); // 11
    }
}
