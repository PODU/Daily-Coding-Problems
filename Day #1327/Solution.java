// Day 1327: Queue backed by a deque of fixed-length blocks (chunks).
// enqueue appends to the tail block (new block when full); dequeue pops from the head block. Amortized O(1).
import java.util.*;

public class Solution {
    static class BlockQueue {
        static final int BLOCK = 4;
        Deque<int[]> blocks = new ArrayDeque<>();
        int tailLen = 0;  // filled count in tail block
        int head = 0;     // read index in head block
        int headLen = 0;  // filled count in head block
        int size = 0;

        void enqueue(int x) {
            if (blocks.isEmpty() || tailLen == BLOCK) {
                blocks.addLast(new int[BLOCK]);
                tailLen = 0;
                if (blocks.size() == 1) headLen = 0;
            }
            blocks.peekLast()[tailLen++] = x;
            if (blocks.size() == 1) headLen = tailLen;
            size++;
        }

        int dequeue() {
            if (size == 0) throw new NoSuchElementException();
            int x = blocks.peekFirst()[head++];
            size--;
            int firstLen = (blocks.size() == 1) ? tailLen : BLOCK;
            if (head == firstLen) {
                blocks.removeFirst();
                head = 0;
            }
            return x;
        }

        int getSize() { return size; }
    }

    public static void main(String[] args) {
        BlockQueue q = new BlockQueue();
        for (int i = 1; i <= 5; i++) q.enqueue(i);
        System.out.println(q.dequeue()); // 1
        System.out.println(q.dequeue()); // 2
        System.out.println(q.getSize()); // 3
    }
}
