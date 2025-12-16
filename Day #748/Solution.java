// Most frequent subtree sum: post-order DFS computes each subtree sum, count in a map.
// Time: O(n), Space: O(n).
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v){ val = v; }
    }

    static int dfs(Node n, Map<Integer,Integer> count){
        if(n == null) return 0;
        int s = n.val + dfs(n.left, count) + dfs(n.right, count);
        count.merge(s, 1, Integer::sum);
        return s;
    }

    static int mostFrequentSubtreeSum(Node root){
        Map<Integer,Integer> count = new HashMap<>();
        dfs(root, count);
        int best = 0, bestCount = -1;
        for(Map.Entry<Integer,Integer> e : count.entrySet())
            if(e.getValue() > bestCount){ bestCount = e.getValue(); best = e.getKey(); }
        return best;
    }

    public static void main(String[] args){
        Node root = new Node(5);
        root.left = new Node(2);
        root.right = new Node(-5);
        System.out.println(mostFrequentSubtreeSum(root)); // 2
    }
}
