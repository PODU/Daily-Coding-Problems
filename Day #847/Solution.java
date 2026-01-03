// Day 847: jump game - can we reach the last index? Greedy furthest-reach. O(n) time, O(1) space.
public class Solution {
    static boolean canReach(int[] a){
        int reach = 0;
        for(int i = 0; i < a.length; i++){
            if(i > reach) return false;
            reach = Math.max(reach, i + a[i]);
        }
        return true;
    }
    public static void main(String[] args){
        System.out.println(canReach(new int[]{2,0,1,0}) ? "True" : "False"); // True
        System.out.println(canReach(new int[]{1,1,0,1}) ? "True" : "False"); // False
    }
}
