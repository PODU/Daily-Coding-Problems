// Day 853: smallest distance (number of words between) between two words in a text.
// Single pass tracking last index of each word. distance = |i-j| - 1. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

int minDistance(const string& text, const string& w1, const string& w2){
    istringstream ss(text);
    string word;
    int idx = 0, p1 = -1, p2 = -1, best = INT_MAX;
    while(ss >> word){
        if(word == w1) p1 = idx;
        if(word == w2) p2 = idx;
        if(p1 != -1 && p2 != -1) best = min(best, abs(p1 - p2) - 1);
        idx++;
    }
    return best;
}

int main(){
    string text = "dog cat hello cat dog dog hello cat world";
    cout << minDistance(text, "hello", "world") << "\n"; // 1
    return 0;
}
