#include <bits/stdc++.h>

using namespace std; 

vector<vector<int>> dist; 

bool cmp(pair<int, int> a, pair<int, int> b)
{
    return dist[a.first][a.second] > dist[b.first][b.second]; 
}

int main()
{
    ifstream in ("in.txt"); 
    vector<vector<int>> risk;
    while(!in.eof())
    {
        string line; 
        getline(in, line); 
        vector<int> row (line.size());
        for(int i = 0; i < line.size(); ++i)
            row[i] = line[i] - '0'; 
        risk.push_back(row); 
    }
    vector<vector<int>> full (5 * risk.size()); 
    for(int i = 0; i < full.size(); ++i)
    {
        full[i].resize(5 * risk[0].size()); 
        for(int j = 0; j < full[i].size(); ++j)
        {
            int m = i % risk.size(); 
            int n = j % risk[0].size();
            int a = i / risk.size(); 
            int b = j / risk[0].size(); 
            full[i][j] = risk[m][n] + (a + b); 
            if(full[i][j] > 9)
                full[i][j] -= 9; 
        }
    }

    dist.resize(full.size()); 
    vector<vector<int>> visit (full.size()); 
    for(int i = 0; i < full.size(); ++i)
    {
        dist[i].resize(full[i].size()); 
        visit[i].resize(full[i].size()); 
        for(int j = 0; j < dist[i].size(); ++j)
            dist[i][j] = numeric_limits<int>::max(); 
    }
    dist[0][0] = 0; 
    priority_queue<pair<int, int>, vector<pair<int, int>>, decltype(&cmp)> pq (cmp); 
    pq.push(pair<int, int>(0, 0)); 
    while(!pq.empty())
    {
        pair<int, int> t = pq.top(); 
        pq.pop(); 
        if(visit[t.first][t.second])
            continue; 
        visit[t.first][t.second] = true; 
        printf("(%d, %d) : %d\n", t.first, t.second, dist[t.first][t.second]); 
        if(t.first > 0 && !visit[t.first-1][t.second])
        {
            int oldDist = dist[t.first-1][t.second]; 
            int newDist = dist[t.first][t.second] + full[t.first-1][t.second]; 
            dist[t.first-1][t.second] = min(oldDist, newDist); 
            pq.push(pair<int, int>(t.first-1, t.second)); 
        }
        if(t.second > 0 && !visit[t.first][t.second-1])
        {
            int oldDist = dist[t.first][t.second-1]; 
            int newDist = dist[t.first][t.second] + full[t.first][t.second-1]; 
            dist[t.first][t.second-1] = min(oldDist, newDist); 
            pq.push(pair<int, int>(t.first, t.second-1)); 
        }
        if(t.first < full.size()-1 && !visit[t.first+1][t.second])
        {
            int oldDist = dist[t.first+1][t.second]; 
            int newDist = dist[t.first][t.second] + full[t.first+1][t.second]; 
            dist[t.first+1][t.second] = min(oldDist, newDist); 
            pq.push(pair<int, int>(t.first+1, t.second)); 
        }
        if(t.second < full[0].size()-1 && !visit[t.first][t.second+1])
        {
            int oldDist = dist[t.first][t.second+1]; 
            int newDist = dist[t.first][t.second] + full[t.first][t.second+1]; 
            dist[t.first][t.second+1] = min(oldDist, newDist); 
            pq.push(pair<int, int>(t.first, t.second+1)); 
        }
    }
    int total = dist[dist.size()-1][dist[0].size()-1]; 
    cout << total << endl; 
}
