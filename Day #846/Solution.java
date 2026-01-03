// Day 846: implement car/cdr for closure-based cons.
// cons stores a pair as a function taking a selector; car selects first, cdr selects second. O(1).
import java.util.function.*;

public class Solution {
    interface Selector { int apply(int a, int b); }
    interface Pair { int apply(Selector f); }

    static Pair cons(int a, int b){ return f -> f.apply(a, b); }
    static int car(Pair p){ return p.apply((a, b) -> a); }
    static int cdr(Pair p){ return p.apply((a, b) -> b); }

    public static void main(String[] args){
        System.out.println(car(cons(3, 4))); // 3
        System.out.println(cdr(cons(3, 4))); // 4
    }
}
