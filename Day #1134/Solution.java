// Running median with two heaps (max-heap low, min-heap high). O(log n) per insert.
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[] nums = {2, 1, 5, 7, 2, 0, 5};
        PriorityQueue<Integer> low = new PriorityQueue<>(Collections.reverseOrder());
        PriorityQueue<Integer> high = new PriorityQueue<>();

        StringBuilder sb = new StringBuilder();
        for (int x : nums) {
            if (low.isEmpty() || x <= low.peek()) low.add(x);
            else high.add(x);
            if (low.size() > high.size() + 1) high.add(low.poll());
            else if (high.size() > low.size()) low.add(high.poll());

            if (low.size() == high.size()) {
                long sum = (long) low.peek() + high.peek();
                if (sum % 2 == 0) sb.append(sum / 2).append("\n");
                else sb.append(sum / 2.0).append("\n");
            } else {
                sb.append(low.peek()).append("\n");
            }
        }
        System.out.print(sb);
    }
}
