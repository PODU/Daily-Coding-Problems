// Day 845: rotate a list left by k in place using the 3-reversal trick.
// reverse(0,k-1), reverse(k,n-1), reverse(0,n-1). O(n) time, O(1) extra space.
import java.util.*;

public class Solution {
    static void reverse(int[] a, int i, int j){
        while(i < j){ int t = a[i]; a[i++] = a[j]; a[j--] = t; }
    }
    static void rotateLeft(int[] a, int k){
        int n = a.length;
        if(n == 0) return;
        k = ((k % n) + n) % n;
        reverse(a, 0, k-1);
        reverse(a, k, n-1);
        reverse(a, 0, n-1);
    }
    public static void main(String[] args){
        int[] a = {1,2,3,4,5,6};
        rotateLeft(a, 2);
        System.out.println(Arrays.toString(a)); // [3, 4, 5, 6, 1, 2]
    }
}
