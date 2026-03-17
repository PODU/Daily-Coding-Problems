// Closure late-binding: closures over a shared mutable holder all read its final value (9);
// fix = copy into an effectively-final local per iteration. O(n) build/call, O(n) space.
import java.util.ArrayList;
import java.util.List;
import java.util.function.Supplier;

public class Solution {
    public static void main(String[] args) {
        // Buggy: all closures read the same holder, which ends at 9.
        int[] shared = {0};
        List<Supplier<Integer>> buggy = new ArrayList<>();
        for (int t = 0; t < 10; t++) {
            shared[0] = t;
            buggy.add(() -> shared[0]);
        }
        for (Supplier<Integer> f : buggy) System.out.println(f.get());

        System.out.println("---");

        // Fixed: a fresh effectively-final local captured per iteration.
        List<Supplier<Integer>> fixed = new ArrayList<>();
        for (int t = 0; t < 10; t++) {
            final int v = t;
            fixed.add(() -> v);
        }
        for (Supplier<Integer> f : fixed) System.out.println(f.get());
    }
}
