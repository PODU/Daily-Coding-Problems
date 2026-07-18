// Smallest positive int not a subset sum of a sorted array. Greedy O(N).
// Keep reachable range [1..res-1]; if a[i] <= res extend, else res is the answer.
public class Solution {
    static long smallestMissing(long[] a){
        long res = 1;
        for(long x : a){
            if(x > res) break;
            res += x;
        }
        return res;
    }
    public static void main(String[] args){
        long[] a = {1, 2, 3, 10};
        System.out.println(smallestMissing(a)); // 7
    }
}
