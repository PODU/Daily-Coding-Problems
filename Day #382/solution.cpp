// Base64 -> bytes -> hex. Bit-accumulator decode (tolerates padding/whitespace).
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

const string B64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

string base64ToHex(const string& s){
    int val[256]; fill(val, val+256, -1);
    for(int i=0;i<64;i++) val[(unsigned char)B64[i]] = i;
    long bits = 0; int nbits = 0;
    string hex;
    const char* hd = "0123456789abcdef";
    for(char c : s){
        int v = val[(unsigned char)c];
        if(v < 0) continue; // skip '=', whitespace, etc.
        bits = (bits << 6) | v; nbits += 6;
        if(nbits >= 8){ nbits -= 8; int byte = (bits >> nbits) & 0xFF; hex += hd[byte>>4]; hex += hd[byte&0xF]; }
    }
    return hex;
}

int main(){
    cout << base64ToHex("3q2+7w=") << "\n";
    return 0;
}
