// Day 1479: Partition a list into <x, ==x, >x around pivot x.
// Stable bucketing into three lists preserves relative order. Time O(N), Space O(N).
import java.util.*;

public class Solution {
    static List<Integer> partition(int[] lst, int x) {
        List<Integer> less = new ArrayList<>(), equal = new ArrayList<>(), greater = new ArrayList<>();
        for (int v : lst) {
            if (v < x) less.add(v);
            else if (v == x) equal.add(v);
            else greater.add(v);
        }
        List<Integer> res = new ArrayList<>(less);
        res.addAll(equal);
        res.addAll(greater);
        return res;
    }

    public static void main(String[] args) {
        System.out.println(partition(new int[]{9, 12, 3, 5, 14, 10, 10}, 10));
        // [9, 3, 5, 10, 10, 12, 14]
    }
}
