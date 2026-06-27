// Reverse words while keeping delimiters in place: split into word/delimiter tokens,
// reverse only the word list, re-emit in original token order. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

string reverseWords(const string& s, const set<char>& delims) {
    vector<string> tokens;   // each token is either a word or a single delimiter char
    vector<bool> isWord;
    string cur;
    for (char c : s) {
        if (delims.count(c)) {
            if (!cur.empty()) { tokens.push_back(cur); isWord.push_back(true); cur.clear(); }
            tokens.push_back(string(1, c)); isWord.push_back(false);
        } else {
            cur += c;
        }
    }
    if (!cur.empty()) { tokens.push_back(cur); isWord.push_back(true); }

    vector<string> words;
    for (size_t i = 0; i < tokens.size(); ++i)
        if (isWord[i]) words.push_back(tokens[i]);
    reverse(words.begin(), words.end());

    string res;
    int wi = 0;
    for (size_t i = 0; i < tokens.size(); ++i)
        res += isWord[i] ? words[wi++] : tokens[i];
    return res;
}

int main() {
    set<char> delims = {'/', ':'};
    cout << reverseWords("hello/world:here", delims) << "\n";
    cout << reverseWords("hello/world:here/", delims) << "\n";
    cout << reverseWords("hello//world:here", delims) << "\n";
    return 0;
}
