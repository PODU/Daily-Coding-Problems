// Apply permutation P: result[P[i]] = array[i]. O(n) time, O(n) space.
// (In-place alternative: follow cycles swapping elements.)
public class Solution {
    static String[] applyPermutation(String[] array, int[] P) {
        String[] result = new String[array.length];
        for (int i = 0; i < array.length; i++) result[P[i]] = array[i];
        return result;
    }

    public static void main(String[] args) {
        String[] array = {"a", "b", "c"};
        int[] P = {2, 1, 0};
        String[] res = applyPermutation(array, P);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.length; i++) {
            sb.append("\"").append(res[i]).append("\"");
            if (i + 1 < res.length) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
