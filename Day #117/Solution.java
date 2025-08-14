// Day 117: BFS level by level, track level with minimum sum. O(n) time, O(w) space.
import java.util.*;

public class Solution {
    static class Node { int val; Node l, r; Node(int v){ val = v; } }

    static int minSumLevel(Node root){
        if (root == null) return -1;
        Queue<Node> q = new LinkedList<>();
        q.add(root);
        int level = 0, best = 0;
        long bestSum = Long.MAX_VALUE;
        while (!q.isEmpty()){
            int sz = q.size();
            long sum = 0;
            for (int i = 0; i < sz; i++){
                Node n = q.poll();
                sum += n.val;
                if (n.l != null) q.add(n.l);
                if (n.r != null) q.add(n.r);
            }
            if (sum < bestSum){ bestSum = sum; best = level; }
            level++;
        }
        return best;
    }

    public static void main(String[] args){
        Node root = new Node(10);
        root.l = new Node(2); root.r = new Node(3);
        root.l.l = new Node(-5); root.l.r = new Node(1);
        System.out.println(minSumLevel(root)); // 2
    }
}
