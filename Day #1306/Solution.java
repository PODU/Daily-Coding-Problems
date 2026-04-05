// Longest absolute file path. Parse tab-indented FS, track cumulative path
// length per depth. Time O(n), Space O(depth).
import java.util.*;

public class Solution {
    public static int lengthLongestPath(String input) {
        String[] lines = input.split("\n", -1);
        Map<Integer,Integer> pathLen = new HashMap<>();
        pathLen.put(0, 0);
        int best = 0;
        for (String l : lines) {
            int level = 0;
            while (level < l.length() && l.charAt(level) == '\t') level++;
            String name = l.substring(level);
            if (name.indexOf('.') >= 0) {
                best = Math.max(best, pathLen.get(level) + name.length());
            } else {
                pathLen.put(level + 1, pathLen.get(level) + name.length() + 1);
            }
        }
        return best;
    }

    public static void main(String[] args) {
        String input = "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext";
        System.out.println(lengthLongestPath(input)); // 32
    }
}
