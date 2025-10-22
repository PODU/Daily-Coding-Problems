// Closure capture: lambdas reading a shared mutable holder all see its final value (9).
// Fix by capturing a per-iteration effectively-final copy (int j = i) -> prints 0..9.
// (Python analogue: lambda: i prints 9 ten times; fix is lambda i=i: i.)
import java.util.*;
import java.util.function.Supplier;

public class Solution {
    public static void main(String[] args) {
        List<Supplier<Integer>> buggy = new ArrayList<>();
        int[] shared = {0};
        for (int i = 0; i < 10; i++) {
            shared[0] = i;
            buggy.add(() -> shared[0]);   // all lambdas read the same shared cell
        }
        System.out.println("Buggy:");
        for (Supplier<Integer> f : buggy) System.out.println(f.get());

        List<Supplier<Integer>> fixed = new ArrayList<>();
        for (int i = 0; i < 10; i++) {
            int j = i;
            fixed.add(() -> j);           // each lambda captures its own copy
        }
        System.out.println("Fixed:");
        for (Supplier<Integer> f : fixed) System.out.println(f.get());
    }
}
