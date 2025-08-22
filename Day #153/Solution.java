// Day 153: Min words separating two words. Single pass tracking last seen index
// of each word; answer is min(|i-j|-1). Time O(n), Space O(1).
public class Solution {
    static int minDistance(String[] words, String a, String b) {
        int lastA = -1, lastB = -1, best = Integer.MAX_VALUE;
        for (int i = 0; i < words.length; i++) {
            if (words[i].equals(a)) {
                lastA = i;
                if (lastB != -1) best = Math.min(best, Math.abs(lastA - lastB) - 1);
            }
            if (words[i].equals(b)) {
                lastB = i;
                if (lastA != -1) best = Math.min(best, Math.abs(lastA - lastB) - 1);
            }
        }
        return best;
    }

    public static void main(String[] args) {
        String text = "dog cat hello cat dog dog hello cat world";
        String[] words = text.split("\\s+");
        System.out.println(minDistance(words, "hello", "world"));
    }
}
