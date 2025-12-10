// Day 730: Closure-in-a-loop late binding (Java analogue).
// Capturing a shared mutable holder -> all lambdas see its final value (3,3,3).
// Fix: capture an effectively-final per-iteration value (1,2,3).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        // Buggy: shared mutable holder
        List<Runnable> buggy = new ArrayList<>();
        int[] holder = new int[1];
        for (int v : new int[]{1, 2, 3}) { holder[0] = v; buggy.add(() -> System.out.println(holder[0])); }
        for (Runnable r : buggy) r.run();   // 3, 3, 3

        // Fixed: per-iteration effectively-final variable
        List<Runnable> fixed = new ArrayList<>();
        for (int v : new int[]{1, 2, 3}) { final int captured = v; fixed.add(() -> System.out.println(captured)); }
        for (Runnable r : fixed) r.run();   // 1, 2, 3
    }
}
