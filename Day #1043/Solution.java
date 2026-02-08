// PeekableIterator: wrap an iterator and cache one element ahead so peek() returns
// the next value without consuming it. O(1) per op, O(1) extra space.
import java.util.Iterator;
import java.util.List;
import java.util.Arrays;

public class Solution {
    static class PeekableInterface {
        private final Iterator<Integer> it;
        private Integer cached = null;

        PeekableInterface(Iterator<Integer> it) { this.it = it; }

        boolean hasNext() { return cached != null || it.hasNext(); }
        int next() {
            if (cached != null) { int v = cached; cached = null; return v; }
            return it.next();
        }
        int peek() {
            if (cached == null) cached = it.next();
            return cached;
        }
    }

    public static void main(String[] args) {
        List<Integer> list = Arrays.asList(1, 2, 3);
        PeekableInterface it = new PeekableInterface(list.iterator());
        System.out.println("peek() -> " + it.peek());
        System.out.println("next() -> " + it.next());
        System.out.println("peek() -> " + it.peek());
        System.out.println("next() -> " + it.next());
        System.out.println("next() -> " + it.next());
        System.out.println("hasNext() -> " + it.hasNext());
    }
}
