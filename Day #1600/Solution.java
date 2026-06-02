// Stack via a single MAX-heap keyed by a monotonic counter; newest key is largest so pops first (LIFO).
// push/pop O(log n), space O(n).
import java.util.*;

public class Solution {
    static class StackViaHeap {
        private final PriorityQueue<long[]> pq =
            new PriorityQueue<>((a, b) -> Long.compare(b[0], a[0])); // max-heap by counter
        private long counter = 0;
        void push(int x) { pq.add(new long[]{counter++, x}); }
        int pop() {
            if (pq.isEmpty()) throw new RuntimeException("pop from empty stack");
            return (int) pq.poll()[1];
        }
        boolean isEmpty() { return pq.isEmpty(); }
    }

    public static void main(String[] args) {
        StackViaHeap s = new StackViaHeap();
        s.push(1); s.push(2); s.push(3);
        List<Integer> out = new ArrayList<>();
        out.add(s.pop()); // 3
        out.add(s.pop()); // 2
        s.push(4);
        out.add(s.pop()); // 4
        out.add(s.pop()); // 1
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < out.size(); i++) {
            if (i > 0) sb.append(' ');
            sb.append(out.get(i));
        }
        System.out.println(sb.toString());
    }
}
