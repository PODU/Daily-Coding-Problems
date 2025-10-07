// Base64 -> bytes -> hex. Bit-accumulator decode (tolerates padding/whitespace).
// Time: O(n), Space: O(n).
public class Solution {
    static final String B64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    static String base64ToHex(String s){
        int[] val = new int[256];
        java.util.Arrays.fill(val, -1);
        for(int i=0;i<64;i++) val[B64.charAt(i)] = i;
        long bits = 0; int nbits = 0;
        StringBuilder hex = new StringBuilder();
        for(int k=0;k<s.length();k++){
            int v = val[s.charAt(k)];
            if(v < 0) continue;
            bits = (bits << 6) | v; nbits += 6;
            if(nbits >= 8){ nbits -= 8; int b = (int)((bits >> nbits) & 0xFF); hex.append(String.format("%02x", b)); }
        }
        return hex.toString();
    }

    public static void main(String[] args){
        System.out.println(base64ToHex("3q2+7w="));
    }
}
