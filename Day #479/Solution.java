// Generate all permutations via backtracking, picking remaining elements in
// index order so output is lexicographic. Time: O(n! * n), Space: O(n) recursion.
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static void backtrack(int[] nums, boolean[] used, List<Integer> cur,
                          List<List<Integer>> res) {
        if (cur.size() == nums.length) {
            res.add(new ArrayList<>(cur));
            return;
        }
        for (int i = 0; i < nums.length; i++) {
            if (used[i]) continue;
            used[i] = true;
            cur.add(nums[i]);
            backtrack(nums, used, cur, res);
            cur.remove(cur.size() - 1);
            used[i] = false;
        }
    }

    public static void main(String[] args) {
        int[] nums = {1, 2, 3};
        List<List<Integer>> res = new ArrayList<>();
        backtrack(nums, new boolean[nums.length], new ArrayList<>(), res);

        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            if (i > 0) sb.append(',');
            sb.append('[');
            for (int j = 0; j < res.get(i).size(); j++) {
                if (j > 0) sb.append(',');
                sb.append(res.get(i).get(j));
            }
            sb.append(']');
        }
        sb.append(']');
        System.out.println(sb.toString());
    }
}
