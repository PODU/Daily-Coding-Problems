// Day 991: Closure late-binding demonstration.
// Original Python prints 3,3,3 (closures capture the loop variable by reference).
// Buggy here shares one mutable cell (all read 3); fixed captures a value copy
// (1,2,3). O(n) time/space.
import java.util.*;
import java.util.function.Supplier;

public class Solution {
    static List<Supplier<Integer>> makeBuggy() {
        List<Supplier<Integer>> v = new ArrayList<>();
        int[] shared = new int[1];                 // single shared cell
        for (int val : new int[]{1, 2, 3}) {
            shared[0] = val;
            v.add(() -> shared[0]);                 // late binding
        }
        return v;
    }

    static List<Supplier<Integer>> makeFixed() {
        List<Supplier<Integer>> v = new ArrayList<>();
        for (int val : new int[]{1, 2, 3}) {
            final int captured = val;               // capture by value
            v.add(() -> captured);
        }
        return v;
    }

    public static void main(String[] args) {
        StringBuilder b = new StringBuilder("Buggy:");
        for (Supplier<Integer> f : makeBuggy()) b.append(' ').append(f.get());
        b.append("\nFixed:");
        for (Supplier<Integer> f : makeFixed()) b.append(' ').append(f.get());
        System.out.println(b);
    }
}
