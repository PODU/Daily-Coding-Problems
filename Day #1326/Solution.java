// Day 1326: Implement reduce/fold — fold an array left to right with a combining function and an initial value.
// O(n) calls to the combiner, O(1) extra space.
import java.util.function.BiFunction;

public class Solution {
    static <T, A> A reduce(T[] lst, BiFunction<A, T, A> combine, A init) {
        A acc = init;
        for (T x : lst) acc = combine.apply(acc, x);
        return acc;
    }

    public static void main(String[] args) {
        Integer[] lst = {1, 2, 3, 4, 5};
        int total = reduce(lst, (Integer a, Integer b) -> a + b, 0);
        System.out.println(total); // 15
    }
}
