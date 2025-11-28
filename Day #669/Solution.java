// Day 669: Misere Nim. First player wins iff either (some heap>1 and xor!=0) or
// (all heaps<=1 and xor==0). Time O(n), Space O(1).
public class Solution {
    static boolean firstPlayerWins(int[] heaps) {
        int x = 0; boolean anyBig = false;
        for (int h : heaps) { x ^= h; if (h > 1) anyBig = true; }
        return anyBig ? (x != 0) : (x == 0);
    }

    public static void main(String[] args) {
        int[] heaps = {3, 4, 5};
        System.out.println(firstPlayerWins(heaps) ? "True" : "False"); // True
    }
}
