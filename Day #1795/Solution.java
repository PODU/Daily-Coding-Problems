// Peekable iterator wrapper: cache one element ahead. peek()/next()/hasNext() O(1) time, O(1) space.
import java.util.Iterator;
import java.util.Arrays;

public class Solution {
    static class Peekable {
        private final Iterator<Integer> it;
        private Integer cached = null;
        Peekable(Iterator<Integer> it) { this.it = it; }
        boolean hasNext() { return cached != null || it.hasNext(); }
        Integer peek() { if (cached == null) cached = it.next(); return cached; }
        Integer next() { if (cached != null) { Integer v = cached; cached = null; return v; } return it.next(); }
    }

    public static void main(String[] args) {
        Peekable p = new Peekable(Arrays.asList(1, 2, 3).iterator());
        System.out.println("peek()    -> " + p.peek());
        System.out.println("next()    -> " + p.next());
        System.out.println("peek()    -> " + p.peek());
        System.out.println("hasNext() -> " + p.hasNext());
        System.out.println("next()    -> " + p.next());
        System.out.println("next()    -> " + p.next());
        System.out.println("hasNext() -> " + p.hasNext());
    }
}
