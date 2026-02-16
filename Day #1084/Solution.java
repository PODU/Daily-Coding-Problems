// reduce/fold: apply combiner left-to-right starting from init. Time O(n), Space O(1).
import java.util.*;
import java.util.function.BiFunction;

public class Solution {
    static <T, A> A reduce(List<T> lst, BiFunction<A, T, A> combine, A init) {
        A acc = init;
        for (T x : lst) acc = combine.apply(acc, x);
        return acc;
    }

    public static void main(String[] a) {
        List<Integer> lst = Arrays.asList(1, 2, 3, 4, 5);
        int s = reduce(lst, (Integer acc, Integer x) -> acc + x, 0);
        System.out.println(s); // 15
    }
}
