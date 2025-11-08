// Closure-based pair: cons stores (a,b) in a functional; car/cdr apply a selector. Time O(1), Space O(1).
import java.util.function.Function;
import java.util.function.IntBinaryOperator;

public class Solution {
    // A Pair is a function taking a selector (a,b)->int and returning int.
    interface Pair extends Function<IntBinaryOperator, Integer> {}

    static Pair cons(int a, int b) {
        return f -> f.applyAsInt(a, b);
    }
    static int car(Pair p) { return p.apply((a, b) -> a); }
    static int cdr(Pair p) { return p.apply((a, b) -> b); }

    public static void main(String[] args) {
        Pair p = cons(3, 4);
        System.out.println(car(p));
        System.out.println(cdr(p));
    }
}
