// Day 154: Stack using only a max-heap. Tag each item with an increasing
// priority so the heap always pops the most recent. push/pop O(log n).
import java.util.*;

public class Solution {
    static class HeapStack {
        // max-heap ordered by priority
        PriorityQueue<long[]> heap =
            new PriorityQueue<>((a, b) -> Long.compare(b[0], a[0]));
        long counter = 0;

        void push(int item) { heap.add(new long[]{counter++, item}); }

        int pop() {
            if (heap.isEmpty()) throw new RuntimeException("pop from empty stack");
            return (int) heap.poll()[1];
        }

        boolean isEmpty() { return heap.isEmpty(); }
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
