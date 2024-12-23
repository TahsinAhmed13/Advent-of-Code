package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type Pair struct {
	first, second int
}

func (p *Pair) Add(q Pair) Pair {
	return Pair{p.first + q.first, p.second + q.second}
}

var deltas = []Pair{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}
var cdeltas = []Pair{{-1, 1}, {1, 1}, {1, -1}, {-1, -1}}

func main() {
	reader := bufio.NewReader(os.Stdin)

	var grid []string
	for {
		line, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		line = strings.TrimSpace(line)
		grid = append(grid, line)
	}

	fmt.Println("Part 1:", part1(grid))
	fmt.Println("Part 2:", part2(grid))
}

func inBounds(pos Pair, bounds Pair) bool {
	return pos.first >= 0 && pos.first < bounds.first && pos.second >= 0 && pos.second < bounds.second
}

func part1(grid []string) int {
	vis := make([][]bool, len(grid))
	for i := range vis {
		vis[i] = make([]bool, len(grid[i]))
	}

	cost := 0
	bounds := Pair{len(grid), len(grid[0])}
	for i := range grid {
		for j := range grid[i] {
			if vis[i][j] {
				continue
			}
			area, perimeter := 0, 0
			stack := []Pair{{i, j}}
			for len(stack) > 0 {
				curr := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				if !inBounds(curr, bounds) || vis[curr.first][curr.second] {
					continue
				}
				vis[curr.first][curr.second] = true
				area++
				for _, d := range deltas {
					next := curr.Add(d)
					if inBounds(next, bounds) && grid[next.first][next.second] == grid[curr.first][curr.second] {
						stack = append(stack, next)
					} else {
						perimeter++
					}
				}
			}
			cost += area * perimeter
		}
	}
	return cost
}

func part2(grid []string) int {
	vis := make([][]bool, len(grid))
	for i := range vis {
		vis[i] = make([]bool, len(grid[i]))
	}

	cost := 0
	bounds := Pair{len(grid), len(grid[0])}
	for i := range grid {
		for j := range grid[i] {
			if vis[i][j] {
				continue
			}
			area, perimeter := 0, 0
			stack := []Pair{{i, j}}
			for len(stack) > 0 {
				curr := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				if !inBounds(curr, bounds) || vis[curr.first][curr.second] {
					continue
				}
				vis[curr.first][curr.second] = true
				area++
				for k := range deltas {
					next := curr.Add(deltas[k])
					adj := curr.Add(deltas[(k+1)%len(deltas)])
					corner := curr.Add(cdeltas[k])

					nextRegion := inBounds(next, bounds) && grid[next.first][next.second] == grid[curr.first][curr.second]
					adjRegion := inBounds(adj, bounds) && grid[adj.first][adj.second] == grid[curr.first][curr.second]
					cornerRegion := inBounds(corner, bounds) && grid[corner.first][corner.second] == grid[curr.first][curr.second]
					isCorner := nextRegion == adjRegion && (!nextRegion || !cornerRegion)

					if nextRegion {
						stack = append(stack, next)
					}
					if isCorner {
						perimeter++
					}
				}
			}
			cost += area * perimeter
		}
	}
	return cost
}
