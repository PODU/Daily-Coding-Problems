// Day 53: FIFO queue from two stacks. Amortized O(1) per op.
// in-stack receives pushes; out-stack serves pops (refilled when empty).
import java.util.*;

public class Solution {
    static class Queue {
        private final Deque<Integer> in = new ArrayDeque<>();
        private final Deque<Integer> out = new ArrayDeque<>();

        void enqueue(int x) { in.push(x); }

        int dequeue() {
            if (out.isEmpty())
                while (!in.isEmpty()) out.push(in.pop());
            return out.pop();
        }
    }

    public static void main(String[] args) {
        Queue q = new Queue();
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);
        System.out.println(q.dequeue()); // 1
        System.out.println(q.dequeue()); // 2
        q.enqueue(4);
        System.out.println(q.dequeue()); // 3
        System.out.println(q.dequeue()); // 4
    }
}
