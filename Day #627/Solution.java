// Peekable iterator: cache one element ahead so peek() returns next without advancing.
// next/peek/hasNext all O(1).
import java.util.Iterator;
import java.util.Arrays;

public class Solution {
    static class Peekable<T> {
        private final Iterator<T> it;
        private T cached;
        private boolean hasCached;

        Peekable(Iterator<T> it) { this.it = it; }

        boolean hasNext() { return hasCached || it.hasNext(); }

        T peek() {
            if (!hasCached) { cached = it.next(); hasCached = true; }
            return cached;
        }

        T next() {
            if (hasCached) { hasCached = false; T v = cached; cached = null; return v; }
            return it.next();
        }
    }

    public static void main(String[] args) {
        Peekable<Integer> it = new Peekable<>(Arrays.asList(1, 2, 3, 4).iterator());
        System.out.println(it.peek());    // 1
        System.out.println(it.next());    // 1
        System.out.println(it.next());    // 2
        System.out.println(it.peek());    // 3
        System.out.println(it.next());    // 3
        System.out.println(it.hasNext()); // true
        System.out.println(it.next());    // 4
        System.out.println(it.hasNext()); // false
    }
}
