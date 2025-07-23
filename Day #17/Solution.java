// Approach: Single pass, track running path length per depth via an array/stack. O(n) time, O(d) space.
import java.util.HashMap;
import java.util.Map;

public class Solution {
    static int lengthLongestPath(String input) {
        Map<Integer, Integer> lenAtDepth = new HashMap<>();
        lenAtDepth.put(-1, 0);
        int maxLen = 0;
        for (String line : input.split("\n")) {
            int depth = 0;
            while (depth < line.length() && line.charAt(depth) == '\t') depth++;
            String name = line.substring(depth);
            boolean isFile = name.indexOf('.') >= 0;
            int curLen = lenAtDepth.get(depth - 1) + name.length();
            if (isFile) {
                maxLen = Math.max(maxLen, curLen);
            } else {
                lenAtDepth.put(depth, curLen + 1); // +1 for '/'
            }
        }
        return maxLen;
    }

    public static void main(String[] args) {
        String input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
        System.out.println(lengthLongestPath(input));
    }
}
