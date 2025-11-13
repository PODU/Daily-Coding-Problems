// Word sense disambiguation via simplified Lesk.
// Score each candidate gloss by content-word overlap with the sentence
// context (other words + their glosses); pick the highest.
// Time O(words * meanings * glossLen). Space O(vocab).
import java.util.*;

public class Solution {
    static final Set<String> STOP = new HashSet<>(Arrays.asList(
        "i","to","the","a","an","of","by","and","or","where","people",
        "that","is","are","in","on","at","with","went","sat","this"));

    static Set<String> tokens(String text) {
        Set<String> out = new HashSet<>();
        for (String w : text.toLowerCase().split("\\s+")) {
            String lw = w.replaceAll("[^a-z]", "");
            if (!lw.isEmpty() && !STOP.contains(lw)) out.add(lw);
        }
        return out;
    }

    public static void main(String[] args) {
        Map<String, List<String>> meanings = new LinkedHashMap<>();
        meanings.put("bank", Arrays.asList(
            "a financial institution where people deposit and withdraw money",
            "the land alongside a river or lake"));
        meanings.put("money", Arrays.asList(
            "currency that people deposit and withdraw"));
        meanings.put("river", Arrays.asList(
            "a large natural stream of water"));

        String[] sentences = {
            "I went to the bank to deposit money",
            "I sat by the bank of the river"
        };

        for (String sentence : sentences) {
            String[] words = sentence.split("\\s+");
            for (String w : words) {
                String lw = w.toLowerCase();
                List<String> cands = meanings.get(lw);
                if (cands == null || cands.size() <= 1) continue;

                Set<String> context = new HashSet<>();
                for (String other : words) {
                    String ol = other.toLowerCase();
                    if (ol.equals(lw)) continue;
                    context.addAll(tokens(other));
                    List<String> og = meanings.get(ol);
                    if (og != null)
                        for (String g : og) context.addAll(tokens(g));
                }

                int best = -1; String bestGloss = "";
                for (String gloss : cands) {
                    int overlap = 0;
                    for (String t : tokens(gloss)) if (context.contains(t)) overlap++;
                    if (overlap > best) { best = overlap; bestGloss = gloss; }
                }
                System.out.println(lw + ": " + bestGloss);
            }
        }
    }
}
