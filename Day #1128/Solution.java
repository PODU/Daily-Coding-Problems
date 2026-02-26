// All permutations of a list of digits in lexicographic order.
// Backtracking over sorted digits. O(n!*n) time, O(n) extra space.
import java.util.*;

public class Solution {
    static void backtrack(int[] digits, boolean[] used, List<Integer> cur, List<List<Integer>> res) {
        if (cur.size() == digits.length) {
            res.add(new ArrayList<>(cur));
            return;
        }
        for (int i = 0; i < digits.length; i++) {
            if (used[i]) continue;
            used[i] = true;
            cur.add(digits[i]);
            backtrack(digits, used, cur, res);
            cur.remove(cur.size() - 1);
            used[i] = false;
        }
    }

    public static void main(String[] args) {
        int[] digits = {1, 2, 3};
        Arrays.sort(digits);
        List<List<Integer>> res = new ArrayList<>();
        backtrack(digits, new boolean[digits.length], new ArrayList<>(), res);

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
