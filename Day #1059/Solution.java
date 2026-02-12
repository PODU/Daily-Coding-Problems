// Word sense disambiguation: for each ambiguous word pick the meaning whose words
// overlap most with the sentence context (other words). Tie-break -> first meaning.
// Time: O(W * M * L), Space: O(L) for the context set.
import java.util.*;

public class Solution {
    static List<String> tokenize(String s) {
        List<String> out = new ArrayList<>();
        for (String t : s.toLowerCase().split("[^a-z0-9]+"))
            if (!t.isEmpty()) out.add(t);
        return out;
    }

    public static void main(String[] args) {
        Map<String, List<String>> dict = new HashMap<>();
        dict.put("bank", Arrays.asList(
            "financial institution where people deposit money",
            "land beside a river or lake"));

        String sentence = "I went to get money from the bank";
        List<String> words = tokenize(sentence);

        for (int i = 0; i < words.size(); i++) {
            String w = words.get(i);
            List<String> meanings = dict.get(w);
            if (meanings == null || meanings.size() <= 1) continue;

            Set<String> context = new HashSet<>();
            for (int j = 0; j < words.size(); j++)
                if (j != i) context.add(words.get(j));

            int best = 0, bestScore = -1;
            for (int m = 0; m < meanings.size(); m++) {
                int score = 0;
                for (String t : tokenize(meanings.get(m)))
                    if (context.contains(t)) score++;
                if (score > bestScore) { bestScore = score; best = m; }
            }
            System.out.println(w + ": " + meanings.get(best));
        }
    }
}
