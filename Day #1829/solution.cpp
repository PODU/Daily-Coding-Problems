// Ghost: build a trie, solve the game bottom-up. A mover wins if some child is
// not a word and is a losing position for the opponent. O(total chars).
#include <bits/stdc++.h>
using namespace std;

struct Trie { map<char,Trie*> ch; bool word = false; };

void insert(Trie* root, const string& w){
    Trie* cur = root;
    for(char c : w){
        if(!cur->ch.count(c)) cur->ch[c] = new Trie();
        cur = cur->ch[c];
    }
    cur->word = true;
}

// can the player about to move at this node force a win?
bool canWin(Trie* node){
    for(auto& kv : node->ch){
        Trie* child = kv.second;
        if(child->word) continue;          // moving here spells a word => mover loses
        if(!canWin(child)) return true;     // opponent loses at child
    }
    return false;
}

int main(){
    vector<string> dict = {"cat","calf","dog","bear"};
    Trie* root = new Trie();
    for(auto& w : dict) insert(root, w);

    vector<char> wins;
    for(auto& kv : root->ch)
        if(!kv.second->word && !canWin(kv.second)) wins.push_back(kv.first);
    sort(wins.begin(), wins.end());

    for(size_t i = 0; i < wins.size(); i++)
        cout << wins[i] << (i + 1 < wins.size() ? ", " : "\n");
    return 0;
}
