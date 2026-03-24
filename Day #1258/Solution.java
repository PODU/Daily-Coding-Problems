// Peeking iterator: buffer one element ahead. peek/next/hasNext all O(1) time, O(1) extra space.
import java.util.Iterator;
import java.util.List;
import java.util.Arrays;

public class Solution {
    static class PeekingIterator implements Iterator<Integer> {
        private final Iterator<Integer> it;
        private Integer buffer = null;
        public PeekingIterator(Iterator<Integer> iterator) { this.it = iterator; }
        public Integer peek() {
            if (buffer == null) buffer = it.next();
            return buffer;
        }
        @Override public Integer next() {
            if (buffer != null) { Integer r = buffer; buffer = null; return r; }
            return it.next();
        }
        @Override public boolean hasNext() { return buffer != null || it.hasNext(); }
    }

    public static void main(String[] args) {
        List<Integer> data = Arrays.asList(1, 2, 3);
        PeekingIterator p = new PeekingIterator(data.iterator());
        System.out.println(p.peek());
        System.out.println(p.next());
        System.out.println(p.next());
        System.out.println(p.peek());
        System.out.println(p.hasNext());
        System.out.println(p.next());
        System.out.println(p.hasNext());
    }
}
