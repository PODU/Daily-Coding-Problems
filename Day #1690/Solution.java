// Boggle solver: build a trie from the dictionary, DFS 8-directionally from each
// cell following trie edges with a visited mask. O(cells * 8^L) worst, pruned by trie.
import java.util.*;

public class Solution {
    static class TrieNode { TrieNode[] ch=new TrieNode[26]; boolean end=false; }

    static char[][] grid;
    static int R, C;
    static boolean[][] vis;
    static TreeSet<String> found=new TreeSet<>();

    static void dfs(int r,int c,TrieNode node,StringBuilder path){
        int idx=grid[r][c]-'a';
        TrieNode nxt=node.ch[idx];
        if(nxt==null) return;
        path.append(grid[r][c]);
        if(nxt.end) found.add(path.toString());
        vis[r][c]=true;
        for(int dr=-1;dr<=1;dr++) for(int dc=-1;dc<=1;dc++){
            if(dr==0&&dc==0) continue;
            int nr=r+dr,nc=c+dc;
            if(nr>=0&&nr<R&&nc>=0&&nc<C&&!vis[nr][nc]) dfs(nr,nc,nxt,path);
        }
        vis[r][c]=false;
        path.deleteCharAt(path.length()-1);
    }

    public static void main(String[] args){
        String[] rows={"oaan","etae","ihkr","iflv"};
        String[] dict={"oath","pea","eat","rain"};

        R=rows.length; C=rows[0].length();
        grid=new char[R][];
        for(int i=0;i<R;i++) grid[i]=rows[i].toCharArray();

        TrieNode root=new TrieNode();
        for(String w:dict){
            TrieNode nd=root;
            for(char ch:w.toCharArray()){ int i=ch-'a'; if(nd.ch[i]==null) nd.ch[i]=new TrieNode(); nd=nd.ch[i]; }
            nd.end=true;
        }

        vis=new boolean[R][C];
        for(int r=0;r<R;r++) for(int c=0;c<C;c++) dfs(r,c,root,new StringBuilder());

        for(String w:found) System.out.println(w);
    }
}
