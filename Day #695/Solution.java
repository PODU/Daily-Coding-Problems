// Day 695: Uniform random in [0, n-1] excluding values in list l.
// Approach: precompute the allowed values once, then pick a uniform index.
// Preprocess O(n), each draw O(1).
import java.util.*;

public class Solution {
    static int[] allowed;
    static Random rng = new Random(42);

    static void build(int n, int[] l) {
        Set<Integer> bad = new HashSet<>();
        for (int x : l) bad.add(x);
        List<Integer> ok = new ArrayList<>();
        for (int x = 0; x < n; x++) if (!bad.contains(x)) ok.add(x);
        allowed = ok.stream().mapToInt(Integer::intValue).toArray();
    }

    static int next() {
        return allowed[rng.nextInt(allowed.length)];
    }

    public static void main(String[] args) {
        build(10, new int[]{2, 4, 6, 8});
        System.out.print("sample: ");
        for (int i = 0; i < 8; i++) System.out.print(next() + " ");
        System.out.println("\n(all values are in [0,9] and never 2,4,6,8)");
    }
}
