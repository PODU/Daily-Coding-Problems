// Day 968: Fewest jumps from 0 to N where i-th jump moves +/- i.
// Approach: smallest k with sum 1..k >= |N| and (sum-|N|) even. Time O(sqrt(N)), Space O(1).
public class Solution {
    static int minJumps(long N) {
        N = Math.abs(N);
        long k = 0, sum = 0;
        while (sum < N || (sum - N) % 2 != 0) { k++; sum += k; }
        return (int) k;
    }

    public static void main(String[] args) {
        System.out.println(minJumps(10)); // 4
    }
}
