// Day 135: Minimum root-to-leaf path sum (with path reconstruction).
// DFS over the tree. O(n) time, O(h) recursion space.
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static class Result {
        int sum;
        List<Integer> path;
        Result(int s, List<Integer> p) { sum = s; path = p; }
    }

    static Result minPath(Node r) {
        if (r == null) return new Result(Integer.MAX_VALUE, new ArrayList<>());
        if (r.left == null && r.right == null)
            return new Result(r.val, new ArrayList<>(List.of(r.val)));
        Result best = new Result(Integer.MAX_VALUE, new ArrayList<>());
        for (Node c : new Node[]{r.left, r.right}) {
            if (c == null) continue;
            Result sub = minPath(c);
            if (sub.sum < best.sum) best = sub;
        }
        best.sum += r.val;
        best.path.add(0, r.val);
        return best;
    }

    public static void main(String[] args) {
        Node root = new Node(10);
        root.left = new Node(5);
        root.left.right = new Node(2);
        root.right = new Node(5);
        root.right.right = new Node(1);
        root.right.right.left = new Node(-1);

        Result res = minPath(root);
        System.out.println(res.sum + " (path " + res.path + ")");
    }
}
