// cons returns a closure pair(f)=f(a,b); car/cdr apply a selector. O(1).
import java.util.function.*;

public class Solution {
    interface Pair {
        int apply(BinaryOperator<Integer> f);
    }

    static Pair cons(int a, int b) {
        return f -> f.apply(a, b);
    }

    static int car(Pair p) {
        return p.apply((a, b) -> a);
    }

    static int cdr(Pair p) {
        return p.apply((a, b) -> b);
    }

    public static void main(String[] args) {
        System.out.println(car(cons(3, 4)));
        System.out.println(cdr(cons(3, 4)));
    }
}
