// Approach: Monotonic deque of indices; front always holds the window max. O(n) time, O(k) space.
import java.util.ArrayDeque;
import java.util.Deque;
import java.util.StringJoiner;

public class Solution {
    static int[] maxSlidingWindow(int[] nums, int k) {
        Deque<Integer> dq = new ArrayDeque<>(); // indices, decreasing values
        int[] res = new int[nums.length - k + 1];
        int idx = 0;
        for (int i = 0; i < nums.length; i++) {
            if (!dq.isEmpty() && dq.peekFirst() <= i - k) dq.pollFirst();
            while (!dq.isEmpty() && nums[dq.peekLast()] <= nums[i]) dq.pollLast();
            dq.offerLast(i);
            if (i >= k - 1) res[idx++] = nums[dq.peekFirst()];
        }
        return res;
    }

    public static void main(String[] args) {
        int[] nums = {10, 5, 2, 7, 8, 7};
        int k = 3;
        int[] res = maxSlidingWindow(nums, k);
        StringJoiner sj = new StringJoiner(", ", "[", "]");
        for (int v : res) sj.add(String.valueOf(v));
        System.out.println(sj.toString());
    }
}
