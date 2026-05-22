// Longest absolute path to a file in a tab-indented filesystem string.
// Track cumulative path length per depth; a name with '.' is a file.
// Time O(n), Space O(depth).
import java.util.*;

public class Solution {
    static int longestPath(String input) {
        Map<Integer,Integer> lens = new HashMap<>();
        lens.put(0, 0);
        int maxLen = 0;
        for (String line : input.split("\n", -1)) {
            int depth = 0;
            while (depth < line.length() && line.charAt(depth) == '\t') depth++;
            String name = line.substring(depth);
            if (name.indexOf('.') >= 0) {
                maxLen = Math.max(maxLen, lens.getOrDefault(depth, 0) + name.length());
            } else {
                lens.put(depth + 1, lens.getOrDefault(depth, 0) + name.length() + 1);
            }
        }
        return maxLen;
    }

    public static void main(String[] args) {
        String input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
        System.out.println(longestPath(input));
    }
}
