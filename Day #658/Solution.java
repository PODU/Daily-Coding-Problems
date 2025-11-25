// Longest absolute file path: split on '\n', track pathLen by depth via array. Time O(n), Space O(depth).
import java.util.*;
public class Solution {
    static int lengthLongestPath(String input) {
        String[] lines = input.split("\n");
        int[] pathLen = new int[lines.length + 1];
        int maxLen = 0;
        for (String line : lines) {
            int depth = 0;
            while (depth < line.length() && line.charAt(depth) == '\t') depth++;
            String name = line.substring(depth);
            int curLen = (depth > 0 ? pathLen[depth - 1] + 1 : 0) + name.length();
            pathLen[depth] = curLen;
            if (name.indexOf('.') != -1)
                maxLen = Math.max(maxLen, curLen);
        }
        return maxLen;
    }
    public static void main(String[] args) {
        String input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
        System.out.println(lengthLongestPath(input));
    }
}
