// Day 96: All permutations via backtracking on the remaining elements, yielding
// lexicographic order. O(n*n!) time.
import java.util.*;

public class Solution {
    static void backtrack(List<Integer> path, List<Integer> rem, List<List<Integer>> res) {
        if (rem.isEmpty()) { res.add(new ArrayList<>(path)); return; }
        for (int i = 0; i < rem.size(); i++) {
            path.add(rem.get(i));
            List<Integer> next = new ArrayList<>(rem);
            next.remove(i);
            backtrack(path, next, res);
            path.remove(path.size() - 1);
        }
    }

    public static void main(String[] args) {
        List<Integer> nums = Arrays.asList(1, 2, 3);
        List<List<Integer>> res = new ArrayList<>();
        backtrack(new ArrayList<>(), new ArrayList<>(nums), res);
        System.out.println(res);
        // [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
    }
}
