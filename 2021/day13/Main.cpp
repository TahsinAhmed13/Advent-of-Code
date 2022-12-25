#include <bits/stdc++.h>

#define MAXX 1311
#define MAXY 895

using namespace std; 

vector<vector<bool>> fold(vector<vector<bool>> dots, char axis, int line)
{
    int m = axis == 'y' ? (dots.size()-1) / 2 : dots.size(); 
    int n = axis == 'x' ? (dots[0].size()-1) / 2 : dots[0].size(); 
    cout << m << " " << n << endl; 
    vector<vector<bool>> out (m); 
    for(int i = 0; i < m; ++i)
    {
        out[i].resize(n); 
        for(int j = 0; j < n; ++j)
        {
            if(axis == 'y')
                out[i][j] = dots[i][j] || dots[dots.size()-i-1][j]; 
            else 
                out[i][j] = dots[i][j] || dots[i][dots[0].size()-j-1]; 
        }
    }
    return out; 
}

int main()
{
    ifstream in ("in.txt"); 
    vector<vector<bool>> dots (MAXY); 
    for(int i = 0; i < MAXY; ++i)
        dots[i].resize(MAXX); 
    cout << MAXY << " " << MAXX << endl; 
    while(!in.eof())
    {
        string s; 
        getline(in, s); 
        if(s.size() < 1)
            break; 
        int x, y; 
        sscanf(s.c_str(), "%d,%d", &x, &y); 
        dots[y][x] = true; 
    }
    while(!in.eof())
    {
        string s;
        getline(in, s); 
        cout << s << endl; 
        char axis; 
        int line; 
        sscanf(s.c_str(), "fold along %c=%d", &axis, &line); 
        dots = fold(dots, axis, line); 
    }

    /*
    int d = 0; 
    for(int i = 0; i < dots.size(); ++i)
        for(int j = 0; j < dots[i].size(); ++j)
            if(dots[i][j])  d++; 
    cout << d << endl; 
    */
    for(int i = 0; i < dots.size(); ++i)
    {
        for(int j = 0; j < dots[i].size(); ++j)
            if(dots[i][j])
                cout << "#"; 
            else 
                cout << "."; 
        cout << endl; 
    }
}
