// Minimum jumps to reach the end. Greedy O(n): track current reachable end & farthest.
// Time: O(n); Space: O(1).
public class Solution {
    static int minJumps(int[] a){
        int n=a.length;
        if(n<=1) return 0;
        int jumps=0, curEnd=0, farthest=0;
        for(int i=0;i<n-1;i++){
            farthest=Math.max(farthest,i+a[i]);
            if(i==curEnd){
                jumps++;
                curEnd=farthest;
                if(curEnd>=n-1) break;
            }
        }
        return jumps;
    }
    public static void main(String[] args){
        int[] a={6,2,4,0,5,1,1,4,2,9};
        System.out.println(minJumps(a));
    }
}
