// Queue built from a deque of fixed-capacity blocks (cap 4). Track head/tail block+offset and an O(1) size.
// enqueue/dequeue/get_size all amortized O(1) time; O(n) space.
import java.util.*;

public class Solution {
    static class BlockQueue {
        static final int CAP = 4;
        Deque<int[]> blocks = new ArrayDeque<>();
        int headOff = 0;
        int tailOff = 0;
        int sz = 0;

        void enqueue(int v) {
            if (blocks.isEmpty() || tailOff == CAP) {
                blocks.addLast(new int[CAP]);
                tailOff = 0;
            }
            blocks.peekLast()[tailOff++] = v;
            sz++;
        }

        int dequeue() {
            int v = blocks.peekFirst()[headOff++];
            sz--;
            if (headOff == CAP) {
                blocks.removeFirst();
                headOff = 0;
            }
            return v;
        }

        int getSize() { return sz; }
    }

    public static void main(String[] args) {
        BlockQueue q = new BlockQueue();
        for (int v : new int[]{1, 2, 3, 4, 5}) q.enqueue(v);
        System.out.println("size=" + q.getSize());
        System.out.println(q.dequeue());
        System.out.println(q.dequeue());
        System.out.println(q.dequeue());
        System.out.println("size=" + q.getSize());
    }
}
