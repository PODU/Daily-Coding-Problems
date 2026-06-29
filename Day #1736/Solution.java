// FIFO queue via two stacks (in/out); dequeue moves in->out when out empty. Amortized O(1) per op, O(n) space.
import java.util.ArrayDeque;
import java.util.Deque;

public class Solution {
    static class MyQueue {
        private final Deque<Integer> in = new ArrayDeque<>();
        private final Deque<Integer> out = new ArrayDeque<>();
        void enqueue(int x) { in.push(x); }
        int dequeue() {
            if (out.isEmpty()) while (!in.isEmpty()) out.push(in.pop());
            return out.pop();
        }
    }

    public static void main(String[] args) {
        MyQueue q = new MyQueue();
        q.enqueue(1); q.enqueue(2); q.enqueue(3);
        System.out.println(q.dequeue());
        q.enqueue(4);
        System.out.println(q.dequeue());
        System.out.println(q.dequeue());
        System.out.println(q.dequeue());
    }
}
