#include <bits/stdc++.h>

using namespace std; 

string digits = "0123456789"; 
vector<vector<int>> grid; 
vector<vector<int>> visit; 

int basin(int i, int j)
{
    if(visit[i][j])
        return 0; 
    visit[i][j] = true; 
    printf("(%d, %d)\n", i, j);
    int s = 1; 
    if(i > 0 && grid[i-1][j] < 9)
        s += basin(i-1, j);  
    if(i < grid.size()-1 && grid[i+1][j] < 9)
        s += basin(i+1, j); 
    if(j > 0 && grid[i][j-1] < 9)
        s += basin(i, j-1); 
    if(j < grid[0].size()-1 && grid[i][j+1] < 9)
        s += basin(i, j+1); 
    return s; 
}

int main()
{
    ifstream in ("in.txt"); 
    while(!in.eof())
    {
        string line;  
        getline(in, line); 
        vector<int> row (line.size()); 
        for(int i = 0; i < line.size(); ++i)
            row[i] = digits.find(line[i]); 
        grid.push_back(row); 
    }
//    int risk = 0; 
    int x, y, z; 
    x = y = z = 1; 
    visit.resize(grid.size()); 
    for(int i = 0; i < grid.size(); ++i)
        visit[i].resize(grid[i].size()); 
    for(int i = 0; i < grid.size(); ++i)
    {
        for(int j = 0; j < grid[i].size(); ++j)
        {
            int t = i > 0 ? grid[i-1][j] : 10; 
            int b = i < grid.size()-1 ? grid[i+1][j] : 10; 
            int l = j > 0 ? grid[i][j-1] : 10; 
            int r = j < grid[i].size()-1 ? grid[i][j+1] : 10; 
            if(grid[i][j] < t && grid[i][j] < b &&
                    grid[i][j] < l && grid[i][j] < r)
            {
//                risk += grid[i][j] + 1; 
                int s = basin(i, j); 
                cout << s << endl; 
                if(x <= y && x <= z && s > x)
                    x = s; 
                else if(y <= x && y <= z && s > y)
                    y = s; 
                else if(z <= x && z <= y && s > z)
                    z = s; 
                cout << x << " " << y << " " << z << endl; 
           }
        }
    }
//    cout << risk << endl; 
    cout << (x * y * z) << endl; 
}
