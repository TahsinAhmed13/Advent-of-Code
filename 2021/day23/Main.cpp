#include <bits/stdc++.h>

using namespace std; 

unordered_map<char, int> energy = {{'A', 1}, {'B', 10}, {'C', 100}, {'D', 1000}}; 

unordered_map<string, int> dist; 
unordered_set<string> visited; 

class Compare
{
public: 
    bool operator() (string a, string b)
    {
        if(dist.count(a) && dist.count(b))
            return dist[a] > dist[b]; 
        else if(dist.count(a))
            return false; 
        else if(dist.count(b))
            return true; 
        else
            return true; 
    }
};  

class NextStates
{
private: 
    int depth; 

    string exch(string s, int i, int j)
    {
        char tmp = s[i]; 
        s[i] = s[j]; 
        s[j] = tmp; 
        return s; 
    }

    bool isCorrect(string state, int t)
    {
        int beg = 7 + t * depth; 
        int i = depth-1; 
        while(i >= 0 && state[beg+i] != '.')
        {
            if(state[beg+i] != 'A'+t)
                return false; 
            i--; 
        }
        return true; 
    }

    int getTop(string state, int t)
    {
        int beg = 7 + t * depth; 
        int i = depth-1; 
        while(i >= 0 && state[beg+i] != '.')
            i--; 
        return i; 
    }

    vector<pair<string, int>> inStates(string state)
    {
        vector<pair<string, int>> in;  
        // go right
        int last = 0; 
        int steps = 1;  
        for(int i = 1; i < 5; ++i)
        {
            if(state[i] != '.')
            {
                last = i; 
                steps = 0; 
            }
            if(state[last] != '.' && state[last]-'A' == i-1 && 
                    isCorrect(state, state[last]-'A'))
            {
                int beg = 7 + (state[last]-'A') * depth; 
                int top = getTop(state, state[last]-'A'); 
                if(top >= 0)
                {
                    string next = exch(state, last, beg+top); 
                    int cost = energy[state[last]] * (steps+top+2); 
                    in.push_back(pair<string, int>(next, cost)); 
                }
            }
            steps += 2; 
        }

        // go left
        last = 6; 
        steps = 1; 
        for(int i = 5; i > 0; --i)
        {
            if(state[i] != '.')
            {
                last = i; 
                steps = 0; 
            }
            if(state[last] != '.' && state[last]-'A' == i-2 && 
                    isCorrect(state, state[last]-'A'))
            {
                int beg = 7 + (state[last]-'A') * depth; 
                int top = getTop(state, state[last]-'A'); 
                if(top >= 0)
                {
                    string next = exch(state, last, beg+top); 
                    int cost = energy[state[last]] * (steps+top+2); 
                    in.push_back(pair<string, int>(next, cost)); 
                }
            }
            steps += 2; 
        }
        return in; 
    }

    vector<pair<string, int>> outStates(string state)
    {
        vector<pair<string, int>> out; 
        for(int i = 0; i < 4; ++i)
        {
            // find top amphiod 
            if(isCorrect(state, i)) 
                continue; 
            int beg = 7 + i * depth; 
            int top = getTop(state, i) + 1; 
            if(top >= depth)
                continue; 
            char type = state[beg+top]; 
                        
            // go left
            int steps = 1; 
            int j = i+1; 
            while(j > 0)
            {
                if(state[j] != '.')
                    break; 
                string next = exch(state, j, beg+top); 
                int cost = energy[type] * (steps+top+1);
                out.push_back(pair<string, int>(next, cost)); 
                steps += 2; 
                j--; 
            }
            if(state[j] == '.')
            {
                out.push_back(pair<string, int>(
                            exch(state, 0, beg+top), 
                            energy[type]*(steps+top)));  
            }

            // go right
            steps = 1; 
            j = i+2; 
            while(j < 6)
            {
                if(state[j] != '.')
                    break; 
                string next = exch(state, j, beg+top); 
                int cost = energy[type] * (steps+top+1);
                out.push_back(pair<string, int>(next, cost)); 
                steps += 2; 
                j++;  
            }
            if(state[j] == '.')
            {
                out.push_back(pair<string, int>(
                            exch(state, 6, beg+top), 
                            energy[type]*(steps+top))); 
            }
        }
        return out; 
    }

public:
    NextStates(int depth) 
    {
        this->depth = depth; 
    }

    vector<pair<string, int>> operator() (string state)
    {
        vector<pair<string, int>> nextStates; 
        for(pair<string, int> s : inStates(state))
            nextStates.push_back(s); 
        for(pair<string, int> s : outStates(state))
            nextStates.push_back(s); 
        return nextStates; 
    }
}; 

int dijkstra(string start, string end,
        NextStates nextStates)
{
    dist.clear(); 
    visited.clear(); 
    dist[start] = 0; 
    priority_queue<string, vector<string>, Compare> pq; 
    pq.push(start); 
    while(!pq.empty())
    {
        string state = pq.top(); 
        pq.pop(); 
        if(visited.count(state))
            continue; 
        visited.insert(state); 
        if(!state.compare(end))
            break; 
        for(pair<string, int> next : nextStates(state))
        {
            if(visited.count(next.first))
                continue; 
            int d = dist[state] + next.second; 
            if(dist.count(next.first))
                dist[next.first] = min(dist[next.first], d); 
            else 
                dist[next.first] = d; 
            pq.push(next.first); 
        }
    }
    return dist[end]; 
}

string getState(vector<string> input)
{
    string state; 
    state += input[1][1]; 
    for(int i = 2; i < input[1].size()-2; i+=2)
        state += input[1][i]; 
    state += input[1][input[1].size()-2]; 
    for(int i = 0; i < 4; ++i)
        for(int j = 2; j < input.size()-1; ++j)
            state += input[j][3+2*i]; 
    return state; 
}

int part1(vector<string> input)
{
    string start = getState(input); 
    string end = ".......AABBCCDD"; 
    return dijkstra(start, end, NextStates(2)); 
}

int part2(vector<string> input)
{
    input.insert(input.end()-2, "  #D#C#B#A#"); 
    input.insert(input.end()-2, "  #D#B#A#C#"); 
    string start = getState(input); 
    string end = ".......AAAABBBBCCCCDDDD"; 
    return dijkstra(start, end, NextStates(4)); 
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
    in.close(); 
    cout << "Part 1: " << part1(input) << endl; 
    cout << "Part 2: " << part2(input) << endl; 
}
