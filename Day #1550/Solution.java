// Closure capture demo: "buggy" closures read a shared mutable holder (prints 9 x10);
// "fixed" closures capture the per-iteration value (prints 0..9). Time O(n), Space O(n).
import java.util.ArrayList;
import java.util.List;
import java.util.function.Supplier;

public class Solution {
    public static void main(String[] args) {
        // Buggy: all closures reference the same mutable holder, whose final value is 9.
        int[] shared = new int[1];
        List<Supplier<Integer>> buggy = new ArrayList<>();
        for (int i = 0; i < 10; i++) {
            shared[0] = i;
            buggy.add(() -> shared[0]);
        }
        for (Supplier<Integer> f : buggy) System.out.println(f.get());

        System.out.println();

        // Fixed: capture the per-iteration value (effectively final copy).
        List<Supplier<Integer>> fixed = new ArrayList<>();
        for (int i = 0; i < 10; i++) {
            final int v = i;
            fixed.add(() -> v);
        }
        for (Supplier<Integer> f : fixed) System.out.println(f.get());
    }
}
