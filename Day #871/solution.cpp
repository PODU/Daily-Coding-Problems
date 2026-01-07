// Soundex phonetic encoding: keep first letter, code rest, dedupe, pad to 3 digits.
// Time O(n), Space O(1) extra per name.
#include <bits/stdc++.h>
using namespace std;

int code(char c){
    c = tolower((unsigned char)c);
    switch(c){
        case 'b': case 'f': case 'p': case 'v': return 1;
        case 'c': case 'g': case 'j': case 'k': case 'q':
        case 's': case 'x': case 'z': return 2;
        case 'd': case 't': return 3;
        case 'l': return 4;
        case 'm': case 'n': return 5;
        case 'r': return 6;
        default:  return 0; // vowels a,e,i,o,u,y
    }
}

string soundex(const string& name){
    int i = 0, n = name.size();
    while(i < n && !isalpha((unsigned char)name[i])) i++;
    if(i >= n) return "";
    string res(1, (char)toupper((unsigned char)name[i]));
    int prev = code(name[i]);
    for(int j = i + 1; j < n && (int)res.size() < 4; j++){
        char c = tolower((unsigned char)name[j]);
        if(!isalpha((unsigned char)c)) continue;
        if(c == 'h' || c == 'w') continue; // transparent
        int d = code(c);
        if(d == 0){ prev = 0; continue; } // vowel resets
        if(d != prev) res += char('0' + d);
        prev = d;
    }
    while(res.size() < 4) res += '0';
    return res.substr(0, 4);
}

int main(){
    cout << soundex("Jackson") << "\n";
    cout << soundex("Jaxen") << "\n";
    return 0;
}
