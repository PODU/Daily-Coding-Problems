// Day 770: Misere Nim forced-win check.
// If every heap == 1: first player wins iff count of heaps is even.
// Else: first player wins iff XOR of heaps != 0. O(N).
public class Solution {
    static boolean firstPlayerWins(int[] heaps) {
        int xorSum = 0;
        boolean allOne = true;
        for (int h : heaps) { xorSum ^= h; if (h > 1) allOne = false; }
        if (allOne) return heaps.length % 2 == 0;
        return xorSum != 0;
    }

    public static void main(String[] args) {
        System.out.println(firstPlayerWins(new int[]{3, 4, 5})); // true
    }
}
