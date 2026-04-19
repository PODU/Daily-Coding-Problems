// FIFO queue via two stacks (in/out); amortized O(1) per op, O(n) space.
import java.util.ArrayDeque;
import java.util.Deque;

public class Solution {
    static Deque<Integer> in = new ArrayDeque<>();
    static Deque<Integer> out = new ArrayDeque<>();

    static void enqueue(int x) { in.push(x); }

    static int dequeue() {
        if (out.isEmpty())
            while (!in.isEmpty()) out.push(in.pop());
        return out.pop();
    }

    public static void main(String[] args) {
        enqueue(1); enqueue(2); enqueue(3);
        System.out.println(dequeue()); // 1
        enqueue(4);
        System.out.println(dequeue()); // 2
        System.out.println(dequeue()); // 3
        System.out.println(dequeue()); // 4
    }
}
