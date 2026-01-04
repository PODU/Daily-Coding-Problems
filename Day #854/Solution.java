// Day 854: greedy word wrap - pack max words per line of length <= k; null if any word > k.
// Single pass over words. O(total characters).
import java.util.*;

public class Solution {
    static List<String> wrap(String s, int k){
        List<String> out = new ArrayList<>();
        StringBuilder line = new StringBuilder();
        for(String word : s.split(" ")){
            if(word.length() > k) return null;
            if(line.length() == 0) line.append(word);
            else if(line.length() + 1 + word.length() <= k) line.append(" ").append(word);
            else { out.add(line.toString()); line = new StringBuilder(word); }
        }
        if(line.length() > 0) out.add(line.toString());
        return out;
    }
    public static void main(String[] args){
        List<String> r = wrap("the quick brown fox jumps over the lazy dog", 10);
        if(r == null) System.out.println("null");
        else {
            StringBuilder sb = new StringBuilder("[");
            for(int i = 0; i < r.size(); i++){ sb.append("\"").append(r.get(i)).append("\""); if(i+1 < r.size()) sb.append(", "); }
            sb.append("]");
            System.out.println(sb); // ["the quick", "brown fox", "jumps over", "the lazy", "dog"]
        }
    }
}
