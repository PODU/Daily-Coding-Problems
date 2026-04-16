// Word circle: backtracking to order all words so each last char == next first char,
// and the last wraps to the first. Time O(n!) worst, Space O(n). (n small)
import java.util.*;

public class Solution {
    static boolean bt(String[] words, List<Integer> order, boolean[] used) {
        if (order.size() == words.length) {
            String first = words[order.get(0)], last = words[order.get(order.size() - 1)];
            return last.charAt(last.length() - 1) == first.charAt(0);
        }
        String lastWord = words[order.get(order.size() - 1)];
        char last = lastWord.charAt(lastWord.length() - 1);
        for (int i = 0; i < words.length; i++) {
            if (!used[i] && words[i].charAt(0) == last) {
                used[i] = true; order.add(i);
                if (bt(words, order, used)) return true;
                order.remove(order.size() - 1); used[i] = false;
            }
        }
        return false;
    }

    static List<Integer> circle(String[] words) {
        boolean[] used = new boolean[words.length];
        List<Integer> order = new ArrayList<>();
        order.add(0); used[0] = true;
        if (bt(words, order, used)) return order;
        return new ArrayList<>();
    }

    public static void main(String[] args) {
        String[] words = {"chair", "height", "racket", "touch", "tunic"};
        List<Integer> order = circle(words);
        if (order.isEmpty()) System.out.println("Cannot form a circle");
        else {
            StringBuilder sb = new StringBuilder();
            for (int idx : order) sb.append(words[idx]).append(" --> ");
            sb.append(words[order.get(0)]);
            System.out.println(sb);
        }
    }
}
