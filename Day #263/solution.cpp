// Day 263: Sentence checker over a stream of characters.
// Approach: scan the stream into candidate sentences (cut at terminal marks),
// validate each with a regex enforcing all four rules. Time O(n) per sentence.

#include <bits/stdc++.h>
using namespace std;

// Rules:
// 1. Starts with a capital letter followed by a lowercase letter or a space.
// 2. Other chars: lowercase letters, separators (, ; :) or terminal marks (. ? ! ‽).
// 3. A single space between each word.
// 4. Ends with a terminal mark immediately following a word.
//
// Pattern (ECMAScript): capital with lookahead for lowercase/space (rule 1),
// a first word, then words separated by single spaces, optional trailing
// separators, ending in a terminal mark. (ASCII terminal marks here; the
// interrobang U+203D is handled in the Unicode-friendly ports.)
static bool isValid(const string &s) {
    static const regex re("^[A-Z](?=[a-z ])[a-z]*[,;:]?( [a-z]+[,;:]?)*[.?!]$");
    return regex_match(s, re);
}

int main() {
    vector<string> tests = {
        "Hello world.",
        "hello world",
        "Hello  world.",
        "The quick, brown fox jumps.",
        "Is this valid?",
        "No terminal mark",
        "Bad ,spacing."
    };
    for (const string &t : tests) {
        if (isValid(t)) cout << t << "\n";
    }
    return 0;
}
