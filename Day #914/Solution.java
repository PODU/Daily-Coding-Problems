// Queue via fixed-length array blocks (capacity 4). Deque of blocks; enqueue to tail, dequeue from head.
// Amortized O(1) per op; O(n) space.
import java.util.ArrayDeque;
import java.util.Deque;

public class Solution {
    static class BlockQueue {
        static final int CAP = 4;
        Deque<int[]> blocks = new ArrayDeque<>();
        int headIdx = 0, tailCount = 0, sz = 0;

        void enqueue(int v) {
            if (blocks.isEmpty() || tailCount == CAP) {
                blocks.addLast(new int[CAP]);
                tailCount = 0;
            }
            blocks.peekLast()[tailCount++] = v;
            sz++;
        }
        int dequeue() {
            if (sz == 0) throw new RuntimeException("empty");
            int v = blocks.peekFirst()[headIdx++];
            sz--;
            if (headIdx == CAP || (blocks.size() == 1 && headIdx == tailCount)) {
                blocks.pollFirst();
                headIdx = 0;
                if (blocks.isEmpty()) tailCount = 0;
            }
            return v;
        }
        int getSize() { return sz; }
    }

    public static void main(String[] args) {
        BlockQueue q = new BlockQueue();
        for (int i = 1; i <= 10; i++) q.enqueue(i);
        StringBuilder sb = new StringBuilder("Dequeued:");
        for (int i = 0; i < 3; i++) sb.append(" ").append(q.dequeue());
        System.out.println(sb);
        System.out.println("Size: " + q.getSize());
    }
}
