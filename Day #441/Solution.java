// Day 441: Three-way partition around pivot x into <x, ==x, >x buckets.
// O(n) time, O(n) space (stable bucket order matches the example).
import java.util.*;

public class Solution {
    static List<Integer> partitionThree(int[] lst, int x) {
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
        int[] lst = {9, 12, 3, 5, 14, 10, 10};
        System.out.println(partitionThree(lst, 10)); // [9, 3, 5, 10, 10, 12, 14]
    }
}
