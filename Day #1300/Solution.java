// Day 1300: Find a contiguous subarray summing to K (handles negatives).
// Prefix-sum hashmap: for each prefix p, look for p-K seen earlier. O(N) time, O(N) space.
import java.util.*;

public class Solution {
    static int[] subarraySum(int[] a, long K) {
        Map<Long, Integer> firstIndex = new HashMap<>();
        firstIndex.put(0L, -1);
        long prefix = 0;
        for (int j = 0; j < a.length; j++) {
            prefix += a[j];
            Integer i = firstIndex.get(prefix - K);
            if (i != null) return Arrays.copyOfRange(a, i + 1, j + 1);
            firstIndex.putIfAbsent(prefix, j);
        }
        return new int[0];
    }

    public static void main(String[] args) {
        System.out.println(Arrays.toString(subarraySum(new int[]{1, 2, 3, 4, 5}, 9))); // [2, 3, 4]
    }
}
