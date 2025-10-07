// Mark covered indices for every substring occurrence, then wrap maximal runs.
// Time: O(|s| * sum|sub|), Space: O(|s|).
public class Solution {
    static String embolden(String s, String[] lst){
        int n = s.length();
        boolean[] bold = new boolean[n];
        for(String sub : lst){
            if(sub.isEmpty()) continue;
            int pos = s.indexOf(sub);
            while(pos != -1){
                for(int i=pos;i<pos+sub.length();i++) bold[i] = true;
                pos = s.indexOf(sub, pos+1);
            }
        }
        StringBuilder out = new StringBuilder();
        for(int i=0;i<n;i++){
            if(bold[i] && (i==0 || !bold[i-1])) out.append("<b>");
            out.append(s.charAt(i));
            if(bold[i] && (i==n-1 || !bold[i+1])) out.append("</b>");
        }
        return out.toString();
    }

    public static void main(String[] args){
        System.out.println(embolden("abcdefg", new String[]{"bc","ef"}));
        System.out.println(embolden("abcdefg", new String[]{"bcd","def"}));
    }
}
