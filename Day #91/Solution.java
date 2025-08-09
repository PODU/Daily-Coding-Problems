// Day 91: Closure-in-loop. Java requires captured vars to be effectively final,
// so the natural fix (per-iteration final copy) captures each value. O(n).
import java.util.*;
import java.util.function.Supplier;

public class Solution {
    public static void main(String[] args) {
        // "Broken" analogue: all share the same final value.
        List<Supplier<Integer>> broken = new ArrayList<>();
        final int[] shared = {0};
        for (int i = 0; i < 10; i++) broken.add(() -> shared[0]);
        shared[0] = 9;

        List<Supplier<Integer>> fixed = new ArrayList<>();
        for (int i = 0; i < 10; i++) {
            final int v = i; // capture current value
            fixed.add(() -> v);
        }

        System.out.print("Broken (prints 9 ten times):");
        for (Supplier<Integer> f : broken) System.out.print(" " + f.get());
        System.out.print("\nFixed:");
        for (Supplier<Integer> f : fixed) System.out.print(" " + f.get());
        System.out.println();
    }
}
