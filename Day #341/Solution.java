// Max non-overlapping dictionary words on a board.
// (1) DFS enumerate every placement (bitmask of cells) per word. (2) Backtrack over
// placements choosing pairwise-disjoint sets to maximize count.
import java.util.*;

public class Solution {
    static int R, C;
    static char[][] board;
    static List<int[]> placements = new ArrayList<>(); // {wordIndex, mask}
    static int bestCount;

    static void dfs(String w, int idx, int r, int c, int used, Set<Integer> masks){
        if(board[r][c] != w.charAt(idx)) return;
        int cell = r*C + c;
        if((used & (1<<cell)) != 0) return;
        used |= (1<<cell);
        if(idx == w.length()-1){ masks.add(used); return; }
        int[] dr={-1,1,0,0}, dc={0,0,-1,1};
        for(int k=0;k<4;k++){
            int nr=r+dr[k], nc=c+dc[k];
            if(nr>=0&&nr<R&&nc>=0&&nc<C) dfs(w, idx+1, nr, nc, used, masks);
        }
    }

    static void backtrack(int start, int occupied, int usedWords, int count){
        bestCount = Math.max(bestCount, count);
        for(int i=start;i<placements.size();i++){
            int w = placements.get(i)[0], m = placements.get(i)[1];
            if((usedWords & (1<<w)) != 0) continue;
            if((occupied & m) != 0) continue;
            backtrack(i+1, occupied|m, usedWords|(1<<w), count+1);
        }
    }

    public static void main(String[] args){
        String[] dict = {"eat","rain","in","rat"};
        board = new char[][]{{'e','a','n'},{'t','t','i'},{'a','r','a'}};
        R = board.length; C = board[0].length;
        for(int wi=0; wi<dict.length; wi++){
            Set<Integer> masks = new HashSet<>();
            for(int r=0;r<R;r++) for(int c=0;c<C;c++) dfs(dict[wi],0,r,c,0,masks);
            for(int m: masks) placements.add(new int[]{wi,m});
        }
        bestCount = 0;
        backtrack(0,0,0,0);
        System.out.println(bestCount);
    }
}
