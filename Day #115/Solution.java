// Day 115: Subtree check via recursive structural match. O(|s|*|t|) worst case.
public class Solution {
    static class Node { int val; Node l, r; Node(int v){ val = v; } }

    static boolean same(Node a, Node b){
        if (a == null && b == null) return true;
        if (a == null || b == null || a.val != b.val) return false;
        return same(a.l, b.l) && same(a.r, b.r);
    }
    static boolean isSubtree(Node s, Node t){
        if (t == null) return true;
        if (s == null) return false;
        if (same(s, t)) return true;
        return isSubtree(s.l, t) || isSubtree(s.r, t);
    }

    public static void main(String[] args){
        Node s = new Node(3);
        s.l = new Node(4); s.r = new Node(5);
        s.l.l = new Node(1); s.l.r = new Node(2);

        Node t = new Node(4); t.l = new Node(1); t.r = new Node(2);
        Node u = new Node(4); u.l = new Node(0);

        System.out.println(isSubtree(s, t)); // true
        System.out.println(isSubtree(s, u)); // false
    }
}
