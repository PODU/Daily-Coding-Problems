// Day 1442: Implement a stack using only a max-heap.
// Approach: tag each item with an increasing counter as its key; the heap's
// max key is always the most recently pushed item. push/pop O(log n).
import java.util.PriorityQueue;

public class Solution {
    static class Stack {
        private final PriorityQueue<long[]> heap =
            new PriorityQueue<>((a, b) -> Long.compare(b[0], a[0])); // max-heap on key
        private long counter = 0;

        void push(int item) { heap.add(new long[]{counter++, item}); }
        int pop() {
            if (heap.isEmpty()) throw new RuntimeException("pop from empty stack");
            return (int) heap.poll()[1];
        }
        boolean isEmpty() { return heap.isEmpty(); }
    }

    public static void main(String[] args) {
        Stack s = new Stack();
        s.push(1); s.push(2); s.push(3);
        System.out.println(s.pop() + " " + s.pop() + " " + s.pop()); // 3 2 1
    }
}
