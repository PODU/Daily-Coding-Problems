// Contiguous subarray summing to K via prefix-sum hash map. O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static int[] subarraySum(int[] a, int K) {
        Map<Long, Integer> seen = new HashMap<>();
        seen.put(0L, -1);
        long pre = 0;
        for (int i = 0; i < a.length; i++) {
            pre += a[i];
            if (seen.containsKey(pre - K)) {
                int start = seen.get(pre - K) + 1;
                return Arrays.copyOfRange(a, start, i + 1);
            }
            seen.putIfAbsent(pre, i);
        }
        return new int[0];
    }

    public static void main(String[] args) {
        System.out.println(Arrays.toString(subarraySum(new int[]{1, 2, 3, 4, 5}, 9)));
    }
}
