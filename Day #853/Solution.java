// Day 853: smallest distance (number of words between) between two words in a text.
// Single pass tracking last index of each word. distance = |i-j| - 1. O(n) time, O(1) space.
public class Solution {
    static int minDistance(String text, String w1, String w2){
        String[] words = text.split(" ");
        int p1 = -1, p2 = -1, best = Integer.MAX_VALUE;
        for(int i = 0; i < words.length; i++){
            if(words[i].equals(w1)) p1 = i;
            if(words[i].equals(w2)) p2 = i;
            if(p1 != -1 && p2 != -1) best = Math.min(best, Math.abs(p1 - p2) - 1);
        }
        return best;
    }
    public static void main(String[] args){
        String text = "dog cat hello cat dog dog hello cat world";
        System.out.println(minDistance(text, "hello", "world")); // 1
    }
}
