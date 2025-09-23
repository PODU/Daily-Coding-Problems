// Smallest window containing all distinct chars: O(n) sliding window.
// Time O(n), Space O(alphabet).
import java.util.*;

public class Solution {
    static int smallestWindow(String s){
        Set<Character> distinct = new HashSet<>();
        for(char c : s.toCharArray()) distinct.add(c);
        int need = distinct.size();
        Map<Character,Integer> cnt = new HashMap<>();
        int have = 0, left = 0, best = Integer.MAX_VALUE;
        for(int right=0; right<s.length(); right++){
            char rc = s.charAt(right);
            cnt.merge(rc, 1, Integer::sum);
            if(cnt.get(rc) == 1) have++;
            while(have == need){
                best = Math.min(best, right - left + 1);
                char lc = s.charAt(left);
                cnt.merge(lc, -1, Integer::sum);
                if(cnt.get(lc) == 0) have--;
                left++;
            }
        }
        return best;
    }

    public static void main(String[] args){
        String s = "jiujitsu";
        System.out.println(smallestWindow(s));
    }
}
