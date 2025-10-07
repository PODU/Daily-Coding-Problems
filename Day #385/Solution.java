// Brute-force all 256 single-byte keys; score by letters/spaces to pick plaintext.
// Time: O(256 * n), Space: O(n).
public class Solution {
    static int[] hexToBytes(String h){
        int[] b = new int[h.length()/2];
        for(int i=0;i<b.length;i++) b[i] = Integer.parseInt(h.substring(2*i, 2*i+2), 16);
        return b;
    }

    static int score(String s){
        for(int i=0;i<s.length();i++){ char c = s.charAt(i); if(c < 32 || c > 126) return -1; }
        int sc = 0;
        for(int i=0;i<s.length();i++){ char c = s.charAt(i); if(Character.isLetter(c) || c==' ') sc++; }
        return sc;
    }

    static String decrypt(String hex){
        int[] bytes = hexToBytes(hex);
        String best = ""; int bestScore = -1;
        for(int k=0;k<256;k++){
            StringBuilder sb = new StringBuilder();
            for(int b : bytes) sb.append((char)(b ^ k));
            String cand = sb.toString();
            int sc = score(cand);
            if(sc > bestScore){ bestScore = sc; best = cand; }
        }
        return best;
    }

    public static void main(String[] args){
        String hex = "7a575e5e5d12455d405e561254405d5f1276535b5e4b12715d565b5c551262405d505e575f";
        System.out.println(decrypt(hex));
    }
}
