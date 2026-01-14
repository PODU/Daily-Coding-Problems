// Stack via max-heap: tag each push with increasing priority; heap pops highest priority (most recent). O(log n)/op, O(n) space.
import java.util.*;

public class Solution {
    static class HeapStack {
        private final PriorityQueue<long[]> pq =
            new PriorityQueue<>((a, b) -> Long.compare(b[0], a[0])); // (priority, value)
        private long counter = 0;
        void push(int v) { pq.offer(new long[]{counter++, v}); }
        int pop() {
            if (pq.isEmpty()) throw new RuntimeException("pop from empty stack");
            return (int) pq.poll()[1];
        }
    }

    public static void main(String[] args) {
        HeapStack s = new HeapStack();
        s.push(1); s.push(2); s.push(3);
        System.out.println(s.pop()); // 3
        System.out.println(s.pop()); // 2
        s.push(4);
        System.out.println(s.pop()); // 4
        System.out.println(s.pop()); // 1
    }
}
