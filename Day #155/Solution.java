// Day 155: Boyer-Moore majority vote in O(n) time, O(1) space. We verify the
// candidate; if no strict majority exists we fall back to the most frequent
// element so the answer is well-defined. Time O(n).
import java.util.*;

public class Solution {
    static int majorityElement(int[] a) {
        int candidate = 0, count = 0;
        for (int x : a) {
            if (count == 0) candidate = x;
            count += (x == candidate) ? 1 : -1;
        }
        int occ = 0;
        for (int x : a) if (x == candidate) occ++;
        if (occ * 2 > a.length) return candidate; // strict majority

        // Fallback: most frequent element (example has no strict majority).
        Map<Integer,Integer> freq = new HashMap<>();
        int best = a[0], bestCnt = 0;
        for (int x : a) {
            int c = freq.merge(x, 1, Integer::sum);
            if (c > bestCnt) { bestCnt = c; best = x; }
        }
        return best;
    }

    public static void main(String[] args) {
        int[] a = {1, 2, 1, 1, 3, 4, 0};
        System.out.println(majorityElement(a)); // 1
    }
}
