// Closure-based pair: cons stores (a,b) in a function; car/cdr apply a selector.
// Time: O(1), Space: O(1) per pair.
import java.util.function.Function;
import java.util.function.BiFunction;

public class Solution {
    interface Pair extends Function<BiFunction<Integer, Integer, Integer>, Integer> {}

    static Pair cons(int a, int b) {
        return f -> f.apply(a, b);
    }
    static int car(Pair p) { return p.apply((a, b) -> a); }
    static int cdr(Pair p) { return p.apply((a, b) -> b); }

    public static void main(String[] args) {
        System.out.println(car(cons(3, 4)));
        System.out.println(cdr(cons(3, 4)));
    }
}
