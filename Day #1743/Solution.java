// Left fold (reduce): single linear pass applying combiner to accumulator. O(n) time, O(1) extra space.
import java.util.function.BiFunction;

public class Solution {
    static <T> T reduce(int[] arr, BiFunction<T, Integer, T> comb, T init) {
        T acc = init;
        for (int x : arr) acc = comb.apply(acc, x);
        return acc;
    }

    public static void main(String[] args) {
        int[] a = {1, 2, 3, 4, 5};
        int s = reduce(a, (acc, x) -> acc + x, 0);
        System.out.println(s);

        int[] b = {1, 2, 3, 4};
        int p = reduce(b, (acc, x) -> acc * x, 1);
        System.out.println(p);
    }
}
