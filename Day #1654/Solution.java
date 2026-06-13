// Simplified Lesk WSD: score each candidate meaning by overlap with the union of other
// in-dict context words and their meaning texts. O(W*M*L) time, O(V) space.
import java.util.*;

public class Solution {
    static List<String> words(String s) {
        return Arrays.asList(s.toLowerCase().split("\\s+"));
    }

    public static void main(String[] a) {
        LinkedHashMap<String, List<String>> meanings = new LinkedHashMap<>();
        meanings.put("bank", Arrays.asList("place where people deposit and withdraw money", "land beside a river or lake"));
        meanings.put("money", Arrays.asList("currency coins and cash used to buy things"));
        meanings.put("river", Arrays.asList("a large natural stream of flowing water"));
        String sentence = "I went to get money from the bank";
        List<String> toks = words(sentence);
        for (String w : toks) {
            if (meanings.containsKey(w) && meanings.get(w).size() > 1) {
                Set<String> ctx = new HashSet<>();
                for (String o : toks) {
                    if (!o.equals(w) && meanings.containsKey(o)) {
                        ctx.add(o);
                        for (String m : meanings.get(o)) ctx.addAll(words(m));
                    }
                }
                String best = meanings.get(w).get(0);
                int bestScore = -1;
                for (String cand : meanings.get(w)) {
                    int score = 0;
                    for (String t : words(cand)) if (ctx.contains(t)) score++;
                    if (score > bestScore) { bestScore = score; best = cand; }
                }
                System.out.println(w + ": " + best);
            }
        }
    }
}
