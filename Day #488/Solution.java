// Day 488: Queue backed by a set of fixed-length arrays (blocks).
// Blocks of size B chained; head/tail indices roll over to next block.
// enqueue/dequeue amortized O(1), get_size O(1). Space O(n).
import java.util.*;

public class Solution {
    static class BlockQueue {
        static final int B = 4;                 // fixed block length
        Deque<int[]> blocks = new ArrayDeque<>(); // set of fixed-length arrays
        int head = 0, tail = 0, sz = 0;

        void enqueue(int x) {
            if (blocks.isEmpty() || tail == B) { // allocate a new fixed block
                blocks.addLast(new int[B]);
                tail = 0;
            }
            blocks.peekLast()[tail++] = x;
            sz++;
        }

        int dequeue() {
            if (sz == 0) throw new NoSuchElementException("empty");
            int x = blocks.peekFirst()[head++];
            sz--;
            if (head == B) {            // front block exhausted -> free it
                blocks.removeFirst();
                head = 0;
            }
            if (blocks.size() == 1 && head == tail) { // single block consumed
                blocks.clear(); head = tail = 0;
            }
            return x;
        }

        int getSize() { return sz; }
    }

    public static void main(String[] args) {
        BlockQueue q = new BlockQueue();
        for (int i = 1; i <= 6; i++) q.enqueue(i); // 1..6
        System.out.println("size=" + q.getSize()); // 6
        System.out.println("deq=" + q.dequeue());   // 1
        System.out.println("deq=" + q.dequeue());   // 2
        q.enqueue(7); q.enqueue(8);
        System.out.println("size=" + q.getSize()); // 6
        StringBuilder sb = new StringBuilder();
        while (q.getSize() > 0) sb.append(q.dequeue()).append(" "); // 3 4 5 6 7 8
        System.out.println(sb.toString().trim());
    }
}
