// Hex string -> bytes -> standard Base64 (with '=' padding).
// Time: O(n), Space: O(n).  Note: canonical Base64 of "deadbeef" is "3q2+7w==".
public class Solution {
    static final String B64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    static int[] hexToBytes(String h){
        int[] out = new int[h.length()/2];
        for(int i=0;i<out.length;i++)
            out[i] = Integer.parseInt(h.substring(2*i, 2*i+2), 16);
        return out;
    }

    static String base64(int[] b){
        StringBuilder out = new StringBuilder();
        for(int i=0;i<b.length;i+=3){
            int rem = b.length - i;
            int n = b[i] << 16;
            if(rem > 1) n |= b[i+1] << 8;
            if(rem > 2) n |= b[i+2];
            out.append(B64.charAt((n>>18)&63));
            out.append(B64.charAt((n>>12)&63));
            out.append(rem > 1 ? B64.charAt((n>>6)&63) : '=');
            out.append(rem > 2 ? B64.charAt(n&63) : '=');
        }
        return out.toString();
    }

    public static void main(String[] args){
        System.out.println(base64(hexToBytes("deadbeef")));
    }
}
