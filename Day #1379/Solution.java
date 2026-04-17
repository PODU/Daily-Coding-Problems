// Max of two without if/comparison: subtract the masked difference using the sign bit.
// max(a,b) = a - ((a-b) & ((a-b)>>63)). Time O(1), Space O(1).
public class Solution {
    static long maxNoBranch(long a, long b) {
        long diff = a - b;
        return a - (diff & (diff >> 63));
    }

    public static void main(String[] args) {
        System.out.println(maxNoBranch(3, 7));   // 7
        System.out.println(maxNoBranch(10, -5)); // 10
    }
}
