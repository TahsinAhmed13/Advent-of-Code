#include <bits/stdc++.h>

using namespace std; 

int step(vector<string>& grid)
{
    vector<string> copy = grid; 
    int moves = 0; 
    for(int i = 0; i < grid.size(); ++i)
    {
        for(int j = 0; j < grid[i].size(); ++j)
        {
            int k = (j + 1) % grid[i].size(); 
            if(copy[i][j] == '>' && copy[i][k] == '.')
            {
                grid[i][j] = '.'; 
                grid[i][k] = '>'; 
                moves++; 
            }
        }
    }
    copy = grid; 
    for(int i = 0; i < grid[0].size(); ++i)
    {
        for(int j = 0; j < grid.size(); ++j)
        {
            int k = (j + 1) % grid.size(); 
            if(copy[j][i] == 'v' && copy[k][i] == '.')
            {
                grid[j][i] = '.'; 
                grid[k][i] = 'v'; 
                moves++; 
            }
        }
    }
    return moves; 
}

int part1(vector<string> grid)
{
    int steps = 1; 
    while(step(grid))
        steps++; 
    return steps; 
}

int main()
{
    ifstream in ("in.txt"); 
    vector<string> input; 
    while(!in.eof())
    {
        string line; 
        getline(in, line); 
        input.push_back(line); 
    }
    cout << "Part 1: " << part1(input) << endl; 
}
