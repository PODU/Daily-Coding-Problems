// Sort characters by descending frequency (ties: first-occurrence order).
// Time: O(n log d), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

string frequencySort(const string& s){
    unordered_map<char,int> cnt, first;
    for(int i=0;i<(int)s.size();i++){ cnt[s[i]]++; if(!first.count(s[i])) first[s[i]] = i; }
    vector<char> chars;
    for(auto& p : cnt) chars.push_back(p.first);
    sort(chars.begin(), chars.end(), [&](char a, char b){
        if(cnt[a] != cnt[b]) return cnt[a] > cnt[b];
        return first[a] < first[b];
    });
    string out;
    for(char c : chars) out.append(cnt[c], c);
    return out;
}

int main(){
    cout << frequencySort("tweet") << "\n";
    return 0;
}
