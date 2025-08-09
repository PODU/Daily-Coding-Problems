// Day 90: Uniform random in [0,n) excluding l. Pick r in [0, n-k), then offset past
// sorted exclusions so every valid value is equally likely. Time O(k log k), Space O(k).
import java.util.*;

public class Solution {
    static int randomExcluding(int n, int[] l, Random rng) {
        TreeSet<Integer> ex = new TreeSet<>();
        for (int v : l) if (v >= 0 && v < n) ex.add(v);
        int m = n - ex.size();
        if (m <= 0) throw new RuntimeException("no valid number");
        int r = rng.nextInt(m);
        for (int e : ex) { if (e <= r) r++; else break; }
        return r;
    }

    public static void main(String[] args) {
        Random rng = new Random(42);
        int n = 5; int[] l = {1, 3};
        System.out.println(randomExcluding(n, l, rng)); // sample from {0,2,4}
    }
}
