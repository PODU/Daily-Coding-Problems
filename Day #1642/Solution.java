// Min word distance: single pass tracking last-seen index of each word; on each
// hit, update min with |i-j|-1 (words strictly between). Time O(n), Space O(1).
public class Solution {
    static int minWordDistance(String[] text, String w1, String w2) {
        int last1 = -1, last2 = -1, best = Integer.MAX_VALUE;
        for (int i = 0; i < text.length; i++) {
            if (text[i].equals(w1)) {
                last1 = i;
                if (last2 != -1) best = Math.min(best, Math.abs(last1 - last2) - 1);
            }
            if (text[i].equals(w2)) {
                last2 = i;
                if (last1 != -1) best = Math.min(best, Math.abs(last1 - last2) - 1);
            }
        }
        return best;
    }

    public static void main(String[] args) {
        String[] text = {"dog","cat","hello","cat","dog","dog","hello","cat","world"};
        System.out.println(minWordDistance(text, "hello", "world"));
    }
}
