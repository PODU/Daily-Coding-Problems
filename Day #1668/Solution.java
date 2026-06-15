// Iterate counting integers; pick those whose digit sum == 10. O(answer) time, O(1) space.
public class Solution {
    static int digitSum(long x){int s=0;while(x>0){s+=x%10;x/=10;}return s;}
    static long nthPerfect(int n){long x=0;int c=0;while(c<n){++x;if(digitSum(x)==10)++c;}return x;}
    public static void main(String[] a){System.out.println(nthPerfect(1));System.out.println(nthPerfect(2));}
}
