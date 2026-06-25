// Unrolled/paged queue: list of fixed-length blocks; head/tail indices. Amortized O(1) per op.
import java.util.*;

public class Solution {
    static class BlockQueue {
        static final int BS = 4;
        Deque<int[]> blocks = new ArrayDeque<>();
        int head = 0, tail = 0, sz = 0;

        void enqueue(int x) {
            if (blocks.isEmpty() || tail == BS) {
                blocks.addLast(new int[BS]);
                tail = 0;
            }
            blocks.peekLast()[tail++] = x;
            sz++;
        }

        int dequeue() {
            if (sz == 0) throw new RuntimeException("empty");
            int x = blocks.peekFirst()[head++];
            sz--;
            if (head == BS || (blocks.size() == 1 && head == tail)) {
                blocks.pollFirst();
                head = 0;
                if (blocks.isEmpty()) tail = 0;
            }
            return x;
        }

        int getSize() { return sz; }
        int numBlocks() { return blocks.size(); }
    }

    public static void main(String[] args) {
        BlockQueue q = new BlockQueue();
        for (int i = 1; i <= 10; i++) q.enqueue(i);
        System.out.println("size after enqueue 1..10: " + q.getSize());
        System.out.println("blocks allocated: " + q.numBlocks());
        System.out.println("dequeue 3: " + q.dequeue() + " " + q.dequeue() + " " + q.dequeue());
        System.out.println("size: " + q.getSize());
        q.enqueue(11);
        System.out.println("enqueue 11, size: " + q.getSize());
        StringBuilder sb = new StringBuilder("dequeue rest:");
        while (q.getSize() > 0) sb.append(" ").append(q.dequeue());
        System.out.println(sb.toString());
        System.out.println("size: " + q.getSize());
    }
}
