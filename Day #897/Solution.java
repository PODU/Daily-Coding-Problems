// Queue via two stacks: enqueue->inStack; dequeue moves all to outStack when empty.
// Amortized O(1) per op, Space O(n).
import java.util.ArrayDeque;
import java.util.Deque;

public class Solution {
    static class Queue {
        private Deque<Integer> inStack = new ArrayDeque<>();
        private Deque<Integer> outStack = new ArrayDeque<>();

        void enqueue(int x) { inStack.push(x); }

        int dequeue() {
            if (outStack.isEmpty()) {
                while (!inStack.isEmpty()) outStack.push(inStack.pop());
            }
            return outStack.pop();
        }
    }

    public static void main(String[] args) {
        Queue q = new Queue();
        q.enqueue(1);
        q.enqueue(2);
        System.out.println(q.dequeue());
        q.enqueue(3);
        System.out.println(q.dequeue());
        System.out.println(q.dequeue());
    }
}
