// Apply permutation: result[P[i]] = array[i]. O(n) time, O(n) space.
public class Solution {
    public static void main(String[] args){
        String[] arr={"a","b","c"};
        int[] P={2,1,0};
        int n=arr.length;
        String[] res=new String[n];
        for(int i=0;i<n;i++) res[P[i]]=arr[i];
        StringBuilder sb=new StringBuilder("[");
        for(int i=0;i<n;i++){ if(i>0) sb.append(", "); sb.append(res[i]); }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
