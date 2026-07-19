// Day 1842: Majority / most-frequent element via a frequency count.
// (Equals the strict majority element whenever one exists.) Time O(N), Space O(N).
import java.util.*;

public class Solution {
    static int majority(int[] a) {
        Map<Integer, Integer> freq = new HashMap<>();
        int best = a[0], bestCount = 0;
        for (int x : a) {
            int c = freq.merge(x, 1, Integer::sum);
            if (c > bestCount) { bestCount = c; best = x; }
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(majority(new int[]{1, 2, 1, 1, 3, 4, 0})); // 1
    }
}
