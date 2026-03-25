// Day 1262: Closure-in-a-loop (late binding) demonstrated in Java.
// Java requires captured variables to be effectively final, so the natural fix
// (a fresh variable per iteration) is enforced and each closure prints its own value.
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        // Emulate the Python bug: a single shared holder mutated by the loop.
        int[] shared = new int[1];
        List<Runnable> buggy = new ArrayList<>();
        List<Runnable> fixed = new ArrayList<>();
        for (int i = 1; i <= 3; i++) {
            shared[0] = i;
            buggy.add(() -> System.out.println(shared[0])); // all read the same holder
            final int captured = i;                          // fresh copy per iteration
            fixed.add(() -> System.out.println(captured));
        }
        System.out.println("Buggy output:");
        for (Runnable r : buggy) r.run();
        System.out.println("Fixed output:");
        for (Runnable r : fixed) r.run();
    }
}
