// Day 188: Closure-in-a-loop late binding (Java analog of the Python snippet).
// Java requires captured locals to be effectively final, so the late-binding bug is
// emulated by sharing a one-element array; the fix snapshots the value per iteration.
// Time/Space O(n).
import java.util.*;
import java.util.function.IntSupplier;

public class Solution {
    static List<IntSupplier> makeFunctionsBuggy() {
        List<IntSupplier> funcs = new ArrayList<>();
        int[] shared = {0};
        for (int v : new int[]{1, 2, 3}) {
            shared[0] = v;
            funcs.add(() -> shared[0]); // all read the final shared value -> 3
        }
        return funcs;
    }

    static List<IntSupplier> makeFunctionsFixed() {
        List<IntSupplier> funcs = new ArrayList<>();
        for (int v : new int[]{1, 2, 3}) {
            final int snap = v;
            funcs.add(() -> snap);
        }
        return funcs;
    }

    public static void main(String[] args) {
        System.out.println("Late binding prints:");
        for (IntSupplier f : makeFunctionsBuggy()) System.out.println(f.getAsInt());
        System.out.println("Fixed prints:");
        for (IntSupplier f : makeFunctionsFixed()) System.out.println(f.getAsInt());
    }
}
