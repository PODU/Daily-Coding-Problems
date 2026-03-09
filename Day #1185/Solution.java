// Day 1185: Smallest distance (words in between) between two words in a text.
// Single pass tracking last index of each target word; min |i-j| - 1.
// Time O(N), Space O(1).
public class Solution {
    static int shortestDistance(String text, String w1, String w2) {
        String[] words = text.split("\\s+");
        int p1 = -1, p2 = -1, best = Integer.MAX_VALUE;
        for (int i = 0; i < words.length; i++) {
            if (words[i].equals(w1)) p1 = i;
            if (words[i].equals(w2)) p2 = i;
            if (p1 >= 0 && p2 >= 0) best = Math.min(best, Math.abs(p1 - p2));
        }
        return best == Integer.MAX_VALUE ? -1 : best - 1;
    }

    public static void main(String[] args) {
        String text = "dog cat hello cat dog dog hello cat world";
        System.out.println(shortestDistance(text, "hello", "world")); // 1
    }
}
