// Sort characters by descending frequency (ties: first-occurrence order).
// Time: O(n log d), Space: O(n).
import java.util.*;

public class Solution {
    static String frequencySort(String s){
        Map<Character,Integer> cnt = new HashMap<>(), first = new HashMap<>();
        for(int i=0;i<s.length();i++){
            char c = s.charAt(i);
            cnt.merge(c, 1, Integer::sum);
            first.putIfAbsent(c, i);
        }
        List<Character> chars = new ArrayList<>(cnt.keySet());
        chars.sort((a,b) -> cnt.get(a).equals(cnt.get(b)) ? first.get(a)-first.get(b) : cnt.get(b)-cnt.get(a));
        StringBuilder out = new StringBuilder();
        for(char c : chars) for(int i=0;i<cnt.get(c);i++) out.append(c);
        return out.toString();
    }

    public static void main(String[] args){
        System.out.println(frequencySort("tweet"));
    }
}
