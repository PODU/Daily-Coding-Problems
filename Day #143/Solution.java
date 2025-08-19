// Partition around pivot into <x, ==x, >x. Stable bucket collect to match expected order. O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static List<Integer> partition(List<Integer> lst, int x) {
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
        List<Integer> lst = Arrays.asList(9, 12, 3, 5, 14, 10, 10);
        System.out.println(partition(lst, 10)); // [9, 3, 5, 10, 10, 12, 14]
    }
}
