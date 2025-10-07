// Brute-force all 256 single-byte keys; score by letters/spaces to pick plaintext.
// Time: O(256 * n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

vector<unsigned char> hexToBytes(const string& h){
    vector<unsigned char> b;
    for(size_t i=0;i+1<h.size();i+=2) b.push_back((unsigned char)stoi(h.substr(i,2), nullptr, 16));
    return b;
}

int score(const string& s){
    for(unsigned char c : s) if(c < 32 || c > 126) return -1; // not printable
    int sc = 0;
    for(unsigned char c : s) if(isalpha(c) || c==' ') sc++;
    return sc;
}

string decrypt(const string& hex){
    auto bytes = hexToBytes(hex);
    string best; int bestScore = -1;
    for(int k=0;k<256;k++){
        string cand(bytes.size(), 0);
        for(size_t i=0;i<bytes.size();i++) cand[i] = bytes[i] ^ k;
        int sc = score(cand);
        if(sc > bestScore){ bestScore = sc; best = cand; }
    }
    return best;
}

int main(){
    string hex = "7a575e5e5d12455d405e561254405d5f1276535b5e4b12715d565b5c551262405d505e575f";
    cout << decrypt(hex) << "\n";
    return 0;
}
