// Three-way stable partition around pivot x: collect <x, ==x, >x in order, concat.
// Time O(n), Space O(n).
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static List<Integer> partition3(int x, int[] lst) {
        List<Integer> less = new ArrayList<>();
        List<Integer> equal = new ArrayList<>();
        List<Integer> greater = new ArrayList<>();
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
        System.out.println(partition3(10, new int[]{9, 12, 3, 5, 14, 10, 10}));
    }
}
