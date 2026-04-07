// Day 1322: Misere Nim (last stone loses) forced win for first player.
// Theorem: first player wins iff (some heap>1 and XOR!=0) OR (all heaps<=1 and XOR==0). O(n) time, O(1) space.
public class Solution {
    static boolean firstPlayerWins(int[] heaps) {
        int x = 0, maxHeap = 0;
        for (int h : heaps) { x ^= h; maxHeap = Math.max(maxHeap, h); }
        if (maxHeap <= 1) return x == 0;
        return x != 0;
    }

    public static void main(String[] args) {
        System.out.println(firstPlayerWins(new int[]{3, 4, 5})); // true
    }
}
