// Day 1109: Three-way (Dutch national flag) partition around pivot x.
// Bucket into <x, ==x, >x preserving relative order to match the example.
// Time: O(N). Space: O(N).
import java.util.*;

public class Solution {
    static List<Integer> partitionThree(int[] lst, int x){
        List<Integer> less = new ArrayList<>(), equal = new ArrayList<>(), greater = new ArrayList<>();
        for (int v : lst){
            if (v < x) less.add(v);
            else if (v == x) equal.add(v);
            else greater.add(v);
        }
        less.addAll(equal);
        less.addAll(greater);
        return less;
    }
    public static void main(String[] args){
        System.out.println(partitionThree(new int[]{9,12,3,5,14,10,10}, 10)); // [9, 3, 5, 10, 10, 12, 14]
    }
}
