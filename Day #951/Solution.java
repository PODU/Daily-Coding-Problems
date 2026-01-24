// Day 951: interleave first half of a stack with the reversed second half,
// in place using only one auxiliary queue (push/pop, enqueue/dequeue).
// Time O(n^2) due to rotations, Space O(1) extra besides the queue.
import java.util.*;

public class Solution {
    static List<Long> interleave(List<Long> input) {
        Deque<Long> st = new ArrayDeque<>(input); // top = peekLast/pollLast
        Deque<Long> q = new ArrayDeque<>();        // queue: offer last, poll first
        while (!st.isEmpty()) q.offer(st.pollLast());
        while (!q.isEmpty()) st.addLast(q.poll());
        while (!st.isEmpty()) q.offer(st.pollLast()); // q = a0..a_{n-1}
        while (!q.isEmpty()) {
            st.addLast(q.poll());
            int m = q.size();
            if (m == 0) break;
            for (int i = 0; i < m - 1; i++) q.offer(q.poll());
            st.addLast(q.poll());
        }
        return new ArrayList<>(st); // bottom..top
    }

    public static void main(String[] args) {
        System.out.println(interleave(Arrays.asList(1L, 2L, 3L, 4L, 5L))); // [1, 5, 2, 4, 3]
        System.out.println(interleave(Arrays.asList(1L, 2L, 3L, 4L)));      // [1, 4, 2, 3]
    }
}
