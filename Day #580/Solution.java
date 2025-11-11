// Min root-to-leaf path sum via DFS, reconstructing the path. Time O(n), Space O(h).
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static class Node { int val; Node left, right; Node(int v){val=v;} }

    static long best;
    static List<Integer> bestPath;

    static void dfs(Node n, List<Integer> cur) {
        if (n == null) return;
        cur.add(n.val);
        if (n.left == null && n.right == null) {
            long s = 0; for (int x : cur) s += x;
            if (s < best) { best = s; bestPath = new ArrayList<>(cur); }
        } else {
            dfs(n.left, cur);
            dfs(n.right, cur);
        }
        cur.remove(cur.size() - 1);
    }

    public static void main(String[] args) {
        Node root = new Node(10);
        root.left = new Node(5);
        root.left.right = new Node(2);
        root.right = new Node(5);
        root.right.right = new Node(1);
        root.right.right.left = new Node(-1);

        best = Long.MAX_VALUE; bestPath = new ArrayList<>();
        dfs(root, new ArrayList<>());

        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < bestPath.size(); i++) {
            sb.append(bestPath.get(i));
            if (i + 1 < bestPath.size()) sb.append(", ");
        }
        sb.append("], which has sum ").append(best).append(".");
        System.out.println(sb.toString());
    }
}
