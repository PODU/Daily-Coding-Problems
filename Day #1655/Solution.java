// Closure capture demo: buggy lambdas read a shared mutable cell (final value);
// fix captures a per-iteration copy. O(n) time/space.
import java.util.*;

public class Solution {
    interface F { int get(); }

    public static void main(String[] a) {
        System.out.println("Late binding (buggy):");
        List<F> funcs = new ArrayList<>();
        int[] shared = new int[1];
        for (int v : new int[]{1, 2, 3}) {
            shared[0] = v;
            funcs.add(() -> shared[0]);
        }
        for (F f : funcs) System.out.println(f.get());
        System.out.println("Fixed (capture value):");
        funcs = new ArrayList<>();
        for (int v : new int[]{1, 2, 3}) {
            final int cap = v;
            funcs.add(() -> cap);
        }
        for (F f : funcs) System.out.println(f.get());
    }
}
