// Day 438: Stack via a max-heap. Each push tags the item with an increasing
// counter; the heap's max counter is the most-recent item. push/pop O(log n).
import java.util.*;

public class Solution {
    static class StackFromHeap {
        // max-heap by counter
        private final PriorityQueue<long[]> heap =
                new PriorityQueue<>((a, b) -> Long.compare(b[0], a[0]));
        private long counter = 0;

        void push(int item) { heap.add(new long[]{counter++, item}); }
        int pop() {
            if (heap.isEmpty()) throw new RuntimeException("stack is empty");
            return (int) heap.poll()[1];
        }
        boolean isEmpty() { return heap.isEmpty(); }
    }

    public static void main(String[] args) {
        StackFromHeap s = new StackFromHeap();
        s.push(1); s.push(2); s.push(3);
        System.out.println(s.pop()); // 3
        System.out.println(s.pop()); // 2
        s.push(4);
        System.out.println(s.pop()); // 4
        System.out.println(s.pop()); // 1
    }
}
