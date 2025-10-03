// Day 363: Curried add_subtract that alternately adds/subtracts arguments.
// Returns a callable that both holds the running value and accepts the next arg.
// Time O(k) per chain of k args, Space O(1).
public class Solution {
    static class AddSubtract {
        final long value;
        final int count;
        AddSubtract(long value, int count) { this.value = value; this.count = count; }
        AddSubtract apply(long x) {
            long v = value + (count % 2 == 1 ? x : -x); // arg1 adds, arg2 subtracts...
            return new AddSubtract(v, count + 1);
        }
        long get() { return value; }
    }

    static AddSubtract addSubtract(long first) { return new AddSubtract(first, 1); }

    public static void main(String[] args) {
        System.out.println(addSubtract(7).get());
        System.out.println("1 + 2 - 3 -> " + addSubtract(1).apply(2).apply(3).get());
        System.out.println("-5 + 10 - 3 + 9 -> " + addSubtract(-5).apply(10).apply(3).apply(9).get());
    }
}
