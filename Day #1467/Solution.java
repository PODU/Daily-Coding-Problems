// All permutations via backtracking, picking remaining elements left-to-right (lexicographic order).
// Time O(n! * n), Space O(n) recursion + output.
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

    static List<List<Integer>> permute(int[] nums) {
        List<List<Integer>> res = new ArrayList<>();
        backtrack(nums, new boolean[nums.length], new ArrayList<>(), res);
        return res;
    }

    public static void main(String[] args) {
        List<List<Integer>> res = permute(new int[]{1, 2, 3});
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
