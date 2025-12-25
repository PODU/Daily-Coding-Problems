// Day 801: nth sevenish number = sum of unique powers of 7.
// Bits of n select which powers of 7 to add (base-7 analog of binary).
// Time O(log n), Space O(1).
public class Solution {
    static long sevenish(int n) {
        long result = 0, power = 1;
        while (n != 0) {
            if ((n & 1) != 0) result += power;
            power *= 7;
            n >>= 1;
        }
        return result;
    }

    public static void main(String[] args) {
        StringBuilder sb = new StringBuilder();
        for (int i = 1; i <= 5; i++) sb.append(sevenish(i)).append(i < 5 ? " " : "");
        System.out.println(sb); // 1 7 8 49 50
    }
}
