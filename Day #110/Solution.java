// Day 110: Root-to-leaf paths via DFS backtracking. O(n) nodes, O(h) stack.
import java.util.*;

public class Solution {
    static class Node { int val; Node l, r; Node(int v){ val = v; } }

    static void dfs(Node n, List<Integer> path, List<List<Integer>> res){
        if (n == null) return;
        path.add(n.val);
        if (n.l == null && n.r == null) res.add(new ArrayList<>(path));
        else { dfs(n.l, path, res); dfs(n.r, path, res); }
        path.remove(path.size() - 1);
    }

    public static void main(String[] args){
        Node root = new Node(1);
        root.l = new Node(2);
        root.r = new Node(3);
        root.r.l = new Node(4);
        root.r.r = new Node(5);
        List<List<Integer>> res = new ArrayList<>();
        dfs(root, new ArrayList<>(), res);
        System.out.println(res);
    }
}
