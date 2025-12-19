// Day 762: Smallest distance (words in between) between two target words.
// Single pass tracking last seen index of each word. Time: O(n), Space: O(1).
public class Solution {
    static int smallestDistance(String[] words, String a, String b) {
        int lastA = -1, lastB = -1, bestGap = Integer.MAX_VALUE;
        for (int i = 0; i < words.length; i++) {
            if (words[i].equals(a)) {
                lastA = i;
                if (lastB != -1) bestGap = Math.min(bestGap, lastA - lastB);
            }
            if (words[i].equals(b)) {
                lastB = i;
                if (lastA != -1) bestGap = Math.min(bestGap, lastB - lastA);
            }
        }
        if (bestGap == Integer.MAX_VALUE) return -1;
        return bestGap - 1;
    }

    public static void main(String[] args) {
        String[] words = {"dog","cat","hello","cat","dog","dog","hello","cat","world"};
        System.out.println(smallestDistance(words, "hello", "world"));  // 1
    }
}
