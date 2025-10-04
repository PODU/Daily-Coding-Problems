// Day 367: Lazily merge two sorted iterators into one sorted iterator.
// Peek the head of each iterator and emit the smaller; never buffers both fully.
// Time O(n+m), Space O(1) beyond the iterators.
import java.util.*;

public class Solution {
    static class MergeIterator implements Iterator<Integer> {
        private final Iterator<Integer> a, b;
        private Integer pa, pb;

        MergeIterator(Iterator<Integer> a, Iterator<Integer> b) {
            this.a = a; this.b = b;
            pa = a.hasNext() ? a.next() : null;
            pb = b.hasNext() ? b.next() : null;
        }

        public boolean hasNext() { return pa != null || pb != null; }

        public Integer next() {
            int v;
            if (pb == null || (pa != null && pa <= pb)) {
                v = pa; pa = a.hasNext() ? a.next() : null;
            } else {
                v = pb; pb = b.hasNext() ? b.next() : null;
            }
            return v;
        }
    }

    public static void main(String[] args) {
        Iterator<Integer> foo = List.of(5, 10, 15).iterator();
        Iterator<Integer> bar = List.of(3, 8, 9).iterator();
        MergeIterator it = new MergeIterator(foo, bar);
        while (it.hasNext()) System.out.println(it.next()); // 3 5 8 9 10 15
    }
}
