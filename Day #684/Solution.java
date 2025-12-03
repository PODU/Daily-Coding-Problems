// Custom reduce/fold (left to right). O(n) time, O(1) extra space.
import java.util.function.IntBinaryOperator;

public class Solution {
    static int reduce(int[] arr, IntBinaryOperator fn, int init) {
        int acc = init;
        for (int x : arr) acc = fn.applyAsInt(acc, x);
        return acc;
    }

    public static void main(String[] args) {
        int[] arr = {1, 2, 3, 4};
        System.out.println(reduce(arr, (a, b) -> a + b, 0)); // 10
        System.out.println(reduce(arr, (a, b) -> a * b, 1)); // 24 (bonus)
    }
}
