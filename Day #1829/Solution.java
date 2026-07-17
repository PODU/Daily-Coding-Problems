// Ghost: build a trie, solve the game bottom-up. A mover wins if some child is
// not a word and is a losing position for the opponent. O(total chars).
import java.util.*;

public class Solution {
    static class Trie { TreeMap<Character,Trie> ch = new TreeMap<>(); boolean word = false; }

    static void insert(Trie root, String w){
        Trie cur = root;
        for(char c : w.toCharArray()){
            cur.ch.putIfAbsent(c, new Trie());
            cur = cur.ch.get(c);
        }
        cur.word = true;
    }

    static boolean canWin(Trie node){
        for(Trie child : node.ch.values()){
            if(child.word) continue;
            if(!canWin(child)) return true;
        }
        return false;
    }

    public static void main(String[] args){
        String[] dict = {"cat","calf","dog","bear"};
        Trie root = new Trie();
        for(String w : dict) insert(root, w);

        List<Character> wins = new ArrayList<>();
        for(Map.Entry<Character,Trie> e : root.ch.entrySet())
            if(!e.getValue().word && !canWin(e.getValue())) wins.add(e.getKey());

        StringBuilder sb = new StringBuilder();
        for(int i = 0; i < wins.size(); i++){
            sb.append(wins.get(i));
            if(i + 1 < wins.size()) sb.append(", ");
        }
        System.out.println(sb.toString());
    }
}
