// Day 1443: Word sense disambiguation (simplified Lesk algorithm).
// For each ambiguous word pick the meaning whose words overlap most with the
// rest of the sentence's words. Time O(W * M * L), Space O(vocab).
import java.util.*;

public class Solution {
    static List<String> tokenize(String s) {
        List<String> out = new ArrayList<>();
        for (String t : s.toLowerCase().split("[^a-z0-9]+"))
            if (!t.isEmpty()) out.add(t);
        return out;
    }

    static Map<String,String> disambiguate(Map<String,List<String>> meanings, String sentence) {
        List<String> words = tokenize(sentence);
        Set<String> context = new HashSet<>(words);
        Map<String,String> result = new LinkedHashMap<>();
        for (String w : words) {
            List<String> senses = meanings.get(w);
            if (senses == null || senses.size() <= 1) continue;
            int best = -1; String bestMeaning = null;
            for (String m : senses) {
                Set<String> mt = new HashSet<>(tokenize(m));
                int score = 0;
                for (String t : mt) if (!t.equals(w) && context.contains(t)) score++;
                if (score > best) { best = score; bestMeaning = m; }
            }
            result.put(w, bestMeaning);
        }
        return result;
    }

    public static void main(String[] args) {
        Map<String,List<String>> meanings = new HashMap<>();
        meanings.put("bank", Arrays.asList(
            "financial institution where people deposit money",
            "sloping land beside a river or lake"));
        String sentence = "I went to the bank to deposit money";
        Map<String,String> res = disambiguate(meanings, sentence);
        System.out.println("bank: " + res.get("bank"));
    }
}
