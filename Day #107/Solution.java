// Day 107: Level-order (BFS) traversal of a binary tree. O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static class Node { int val; Node l, r; Node(int v){ val = v; } }

    static List<Integer> levelOrder(Node root){
        List<Integer> out = new ArrayList<>();
        if (root == null) return out;
        Queue<Node> q = new LinkedList<>();
        q.add(root);
        while (!q.isEmpty()){
            Node n = q.poll();
            out.add(n.val);
            if (n.l != null) q.add(n.l);
            if (n.r != null) q.add(n.r);
        }
        return out;
    }

    public static void main(String[] args){
        Node root = new Node(1);
        root.l = new Node(2);
        root.r = new Node(3);
        root.r.l = new Node(4);
        root.r.r = new Node(5);
        List<Integer> v = levelOrder(root);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < v.size(); i++){
            sb.append(v.get(i));
            if (i + 1 < v.size()) sb.append(", ");
        }
        System.out.println(sb.toString());
    }
}
