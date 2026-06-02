// Reconstruct binary tree from preorder+inorder using inorder index hashmap
// and a moving preorder pointer. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static class Node { char val; Node left, right; Node(char v){ val=v; } }

    static Map<Character,Integer> idx = new HashMap<>();
    static int preIdx;
    static char[] pre;

    static Node build(int inL, int inR) {
        if (inL > inR) return null;
        char rootVal = pre[preIdx++];
        Node root = new Node(rootVal);
        int mid = idx.get(rootVal);
        root.left  = build(inL, mid - 1);
        root.right = build(mid + 1, inR);
        return root;
    }

    static void preorder(Node n, StringBuilder sb){ if(n==null)return; if(sb.length()>0)sb.append(' '); sb.append(n.val); preorder(n.left,sb); preorder(n.right,sb); }
    static void inorder(Node n, StringBuilder sb){ if(n==null)return; inorder(n.left,sb); if(sb.length()>0)sb.append(' '); sb.append(n.val); inorder(n.right,sb); }
    static void postorder(Node n, StringBuilder sb){ if(n==null)return; postorder(n.left,sb); postorder(n.right,sb); if(sb.length()>0)sb.append(' '); sb.append(n.val); }

    public static void main(String[] args) {
        pre = new char[]{'a','b','d','e','c','f','g'};
        char[] in = {'d','b','e','a','f','c','g'};
        for (int i = 0; i < in.length; i++) idx.put(in[i], i);
        preIdx = 0;
        Node root = build(0, in.length - 1);

        StringBuilder po = new StringBuilder(), pr = new StringBuilder(), io = new StringBuilder();
        postorder(root, po);
        preorder(root, pr);
        inorder(root, io);
        System.out.println("postorder: " + po);
        System.out.println("preorder:  " + pr);
        System.out.println("inorder:   " + io);
    }
}
