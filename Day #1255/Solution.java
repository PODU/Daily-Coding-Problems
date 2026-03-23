// Two-sum existence: one-pass hash set, check (k-num) seen before insert.
// Time O(n), Space O(n).
import java.util.*;
public class Solution {
    static boolean hasPair(int[] a,int k){
        Set<Integer> seen=new HashSet<>();
        for(int x:a){ if(seen.contains(k-x))return true; seen.add(x); }
        return false;
    }
    public static void main(String[] args){
        int[] v={10,15,3,7};
        System.out.println(hasPair(v,17));
        System.out.println(hasPair(v,50));
    }
}
