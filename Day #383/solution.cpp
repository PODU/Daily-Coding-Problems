// Mark covered indices for every substring occurrence, then wrap maximal runs.
// Time: O(|s| * sum|sub|), Space: O(|s|).
#include <bits/stdc++.h>
using namespace std;

string embolden(const string& s, const vector<string>& lst){
    int n = s.size();
    vector<bool> bold(n, false);
    for(const string& sub : lst){
        if(sub.empty()) continue;
        size_t pos = s.find(sub, 0);
        while(pos != string::npos){
            for(size_t i=pos;i<pos+sub.size();i++) bold[i] = true;
            pos = s.find(sub, pos+1);
        }
    }
    string out;
    for(int i=0;i<n;i++){
        if(bold[i] && (i==0 || !bold[i-1])) out += "<b>";
        out += s[i];
        if(bold[i] && (i==n-1 || !bold[i+1])) out += "</b>";
    }
    return out;
}

int main(){
    cout << embolden("abcdefg", {"bc","ef"}) << "\n";
    cout << embolden("abcdefg", {"bcd","def"}) << "\n";
    return 0;
}
