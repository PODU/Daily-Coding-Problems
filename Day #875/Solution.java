// Longest absolute file path: track path length per depth in one pass.
// Time O(n), Space O(d) for depth d.
import java.util.*;

public class Solution {
    static int lengthLongestPath(String input){
        Map<Integer, Integer> pathLen = new HashMap<>();
        pathLen.put(0, 0);
        int maxLen = 0;
        for(String line : input.split("\n")){
            int depth = 0;
            while(depth < line.length() && line.charAt(depth) == '\t') depth++;
            String name = line.substring(depth);
            if(name.indexOf('.') >= 0){
                maxLen = Math.max(maxLen, pathLen.getOrDefault(depth, 0) + name.length());
            } else {
                pathLen.put(depth + 1, pathLen.getOrDefault(depth, 0) + name.length() + 1);
            }
        }
        return maxLen;
    }

    public static void main(String[] args){
        String input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
        System.out.println(lengthLongestPath(input));
    }
}
