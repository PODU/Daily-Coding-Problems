// Greedy line wrapping: fit max words per line within width k.
// Return null (empty optional) if any single word exceeds k.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

bool wrap(const string& s, int k, vector<string>& out){
    vector<string> words;
    stringstream ss(s); string w;
    while(ss >> w) words.push_back(w);
    string line;
    for(auto& word : words){
        if((int)word.size() > k) return false;
        if(line.empty()) line = word;
        else if((int)line.size() + 1 + (int)word.size() <= k) line += " " + word;
        else { out.push_back(line); line = word; }
    }
    if(!line.empty()) out.push_back(line);
    return true;
}

int main(){
    vector<string> out;
    bool ok = wrap("the quick brown fox jumps over the lazy dog", 10, out);
    if(!ok){ cout << "null" << endl; return 0; }
    cout << "[";
    for(size_t i=0;i<out.size();i++){ cout << "\"" << out[i] << "\""; if(i+1<out.size()) cout << ", "; }
    cout << "]" << endl;
    // ["the quick", "brown fox", "jumps over", "the lazy", "dog"]
    return 0;
}
