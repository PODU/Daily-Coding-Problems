// Simplified Lesk: score each gloss by word overlap with sentence context; pick max (ties->first).
// Time O(words * meanings * glossLen), Space O(vocab).
import java.util.*;

public class Solution {
    static Set<String> tokSet(String s) {
        Set<String> set = new HashSet<>();
        for (String w : s.toLowerCase().trim().split("\\s+"))
            if (!w.isEmpty()) set.add(w);
        return set;
    }

    public static void main(String[] args) {
        Map<String, List<String>> meanings = new HashMap<>();
        meanings.put("bank", Arrays.asList(
            "place where people deposit and withdraw money",
            "sloping land beside a river or lake of water"));
        meanings.put("money", Arrays.asList("currency cash that people deposit"));
        meanings.put("river", Arrays.asList("large natural stream of water"));

        String sentence = "I went to get money from the bank";
        String[] tokens = sentence.toLowerCase().trim().split("\\s+");

        for (int i = 0; i < tokens.length; i++) {
            List<String> senses = meanings.get(tokens[i]);
            if (senses == null || senses.size() < 2) continue; // not ambiguous
            Set<String> context = new HashSet<>();
            for (int j = 0; j < tokens.length; j++) if (j != i) context.add(tokens[j]);
            int bestIdx = 0, bestScore = -1;
            for (int idx = 0; idx < senses.size(); idx++) {
                int score = 0;
                for (String w : tokSet(senses.get(idx))) if (context.contains(w)) score++;
                if (score > bestScore) { bestScore = score; bestIdx = idx; }
            }
            System.out.println(tokens[i] + ": " + senses.get(bestIdx));
        }
    }
}
