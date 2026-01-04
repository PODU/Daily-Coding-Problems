// Day 854: greedy word wrap - pack max words per line of length <= k; null if any word > k.
// Single pass over words. O(total characters).
#include <bits/stdc++.h>
using namespace std;

// returns false (null) if impossible
bool wrap(const string& s, int k, vector<string>& out){
    istringstream ss(s);
    string word, line;
    while(ss >> word){
        if((int)word.size() > k) return false;            // word can't fit any line
        if(line.empty()) line = word;
        else if((int)(line.size() + 1 + word.size()) <= k) line += " " + word;
        else { out.push_back(line); line = word; }
    }
    if(!line.empty()) out.push_back(line);
    return true;
}

int main(){
    vector<string> out;
    if(wrap("the quick brown fox jumps over the lazy dog", 10, out)){
        cout << "[";
        for(size_t i = 0; i < out.size(); ++i){ cout << "\"" << out[i] << "\""; if(i+1 < out.size()) cout << ", "; }
        cout << "]\n"; // ["the quick", "brown fox", "jumps over", "the lazy", "dog"]
    } else cout << "null\n";
    return 0;
}
