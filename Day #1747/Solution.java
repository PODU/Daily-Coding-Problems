// Day 1747: All permutations of a list of digits.
// Approach: backtracking with a used[] mask, iterating values in order -> lexicographic.
// Time O(n * n!), space O(n) recursion (plus O(n!) for the output).
import java.util.*;

public class Solution {
    static void backtrack(int[] nums, boolean[] used, List<Integer> cur,
                          List<List<Integer>> res) {
        if (cur.size() == nums.length) { res.add(new ArrayList<>(cur)); return; }
        for (int i = 0; i < nums.length; i++) {
            if (used[i]) continue;
            used[i] = true;
            cur.add(nums[i]);
            backtrack(nums, used, cur, res);
            cur.remove(cur.size() - 1);
            used[i] = false;
        }
    }

    static List<List<Integer>> permutations(int[] nums) {
        List<List<Integer>> res = new ArrayList<>();
        backtrack(nums, new boolean[nums.length], new ArrayList<>(), res);
        return res;
    }

    public static void main(String[] args) {
        int[] nums = {1, 2, 3};
        List<List<Integer>> res = permutations(nums);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            sb.append("[");
            for (int j = 0; j < res.get(i).size(); j++) {
                sb.append(res.get(i).get(j));
                if (j + 1 < res.get(i).size()) sb.append(",");
            }
            sb.append("]");
            if (i + 1 < res.size()) sb.append(",");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
