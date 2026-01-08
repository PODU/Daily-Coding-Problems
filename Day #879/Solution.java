// Two numbers summing to k via one-pass hash set. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static boolean twoSum(int[] nums, int k){
        Set<Integer> seen = new HashSet<>();
        for(int x : nums){
            if(seen.contains(k - x)) return true;
            seen.add(x);
        }
        return false;
    }

    public static void main(String[] args){
        int[] nums = {10, 15, 3, 7};
        System.out.println(twoSum(nums, 17));
    }
}
