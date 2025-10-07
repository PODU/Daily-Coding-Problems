// Hex string -> bytes -> standard Base64 (with '=' padding).
// Time: O(n), Space: O(n).  Note: canonical Base64 of "deadbeef" is "3q2+7w==".
#include <bits/stdc++.h>
using namespace std;

const string B64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

vector<unsigned char> hexToBytes(const string& h){
    vector<unsigned char> out;
    for(size_t i=0;i+1<h.size();i+=2)
        out.push_back((unsigned char)stoi(h.substr(i,2), nullptr, 16));
    return out;
}

string base64(const vector<unsigned char>& b){
    string out;
    for(size_t i=0;i<b.size();i+=3){
        int n = (int)b[i] << 16;
        int rem = b.size() - i;
        if(rem > 1) n |= b[i+1] << 8;
        if(rem > 2) n |= b[i+2];
        out += B64[(n>>18)&63];
        out += B64[(n>>12)&63];
        out += (rem > 1) ? B64[(n>>6)&63] : '=';
        out += (rem > 2) ? B64[n&63] : '=';
    }
    return out;
}

int main(){
    string hex = "deadbeef";
    cout << base64(hexToBytes(hex)) << "\n";
    return 0;
}
