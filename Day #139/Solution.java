// Wrap an iterator and buffer one element for peek(). next/hasNext/peek all O(1).
// Time O(1) per op; Space O(1).
import java.util.Iterator;
import java.util.List;
import java.util.Arrays;

public class Solution {
    static class PeekableInterface {
        private final Iterator<Integer> it;
        private Integer buffer = null;

        PeekableInterface(Iterator<Integer> iterator) { this.it = iterator; }

        int peek() {
            if (buffer == null) buffer = it.next();
            return buffer;
        }

        int next() {
            if (buffer != null) { int v = buffer; buffer = null; return v; }
            return it.next();
        }

        boolean hasNext() { return buffer != null || it.hasNext(); }
    }

    public static void main(String[] args) {
        List<Integer> data = Arrays.asList(1, 2, 3);
        PeekableInterface p = new PeekableInterface(data.iterator());
        System.out.println("peek=" + p.peek() + " next=" + p.next()
            + " peek=" + p.peek() + " next=" + p.next()
            + " next=" + p.next() + " hasNext=" + p.hasNext());
        // peek=1 next=1 peek=2 next=2 next=3 hasNext=false
    }
}
