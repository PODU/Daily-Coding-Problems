// Day 1118 - Late-binding closure pitfall
// Closing over shared mutable state (holder) makes every lambda see its final
// value (9). Fix: capture an effectively-final per-iteration copy.
import java.util.*;
import java.util.function.Supplier;

public class Solution {
    public static void main(String[] args) {
        List<Supplier<Integer>> buggy = new ArrayList<>();
        List<Supplier<Integer>> fixed = new ArrayList<>();
        int[] holder = {0};
        for (int k = 0; k < 10; k++) {
            holder[0] = k;
            buggy.add(() -> holder[0]); // shared mutable state
            final int j = k;
            fixed.add(() -> j);         // own copy
        }

        System.out.println("Buggy output (all 9):");
        for (Supplier<Integer> f : buggy) System.out.println(f.get());
        System.out.println("Fixed output (0-9):");
        for (Supplier<Integer> f : fixed) System.out.println(f.get());
    }
}
