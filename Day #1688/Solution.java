// cons returns a closure taking a selector(a,b)->int; car/cdr pass selectors
// returning first/second arg. All O(1).
import java.util.function.Function;
import java.util.function.IntBinaryOperator;

public class Solution {
    // A Pair is a function: given a selector (a,b)->int, returns its result.
    interface Pair extends Function<IntBinaryOperator, Integer> {}

    static Pair cons(int a, int b) {
        return f -> f.applyAsInt(a, b);
    }

    static int car(Pair p) { return p.apply((a, b) -> a); }
    static int cdr(Pair p) { return p.apply((a, b) -> b); }

    public static void main(String[] args) {
        System.out.println(car(cons(3, 4)));
        System.out.println(cdr(cons(3, 4)));
    }
}
