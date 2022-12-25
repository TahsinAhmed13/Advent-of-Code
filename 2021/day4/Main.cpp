#include <bits/stdc++.h>

using namespace std;

pair<int, int> find(vector<vector<int>> board, int n)
{
    pair<int, int> out; 
    out.first = -1; 
    out.second = -1; 
    for(int i = 0; i < 5; ++i)
    {
        for(int j = 0; j < 5; ++j)
        {
            if(n == board[i][j])
            {
                out.first = i; 
                out.second = j; 
                return out; 
            }
        }
    }
    return out; 
}

pair<int, int> play(vector<int> draw, vector<vector<int>> board, int total)
{
    vector<int> rows (5, 0); 
    vector<int> cols (5, 0); 
    int n = 0; 
    int s = -1; 
    while(n < draw.size())
    {
        pair<int, int> loc = find(board, draw[n]); 
        if(loc.first != -1)
        {
            total -= draw[n]; 
            rows[loc.first]++; 
            if(rows[loc.first] == 5)
            {
                s = total * draw[n]; 
                break; 
            }
            cols[loc.second]++; 
            if(cols[loc.second] == 5)
            {
                s = total * draw[n]; 
                break; 
            }
        }
        n++; 
    }
    pair<int, int> out; 
    out.first = n+1; 
    out.second = s; 
    return out; 
}

int main()
{
    ifstream in ("in.txt"); 
    vector<int> draw; 
    int n; 
    in >> n; 
    draw.push_back(n); 
    while(in.peek() == ',')
    {
        char foo; 
        in >> foo; 
        in >> n; 
        draw.push_back(n); 
    }
//    int fast = draw.size();
    int fast = 0; 
    int score = 0; 
    vector<vector<int>> board (5); 
    for(int i = 0; i < 5; ++i)
        board[i].resize(5); 
    while(!in.eof())
    {
        int t = 0; 
        for(int i = 0; i < 5; ++i)
        {
            for(int j = 0; j < 5; ++j)
            {
                in >> board[i][j]; 
                t += board[i][j]; 
            }
        }
        pair<int, int> res = play(draw, board, t); 
        // if(res.second != -1 && res.first < fast)
        if(res.second != -1 && res.first > fast)
        {
            fast = res.first; 
            score = res.second; 
        }
    }
    cout << score << endl; 
}
