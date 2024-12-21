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

var deltas = []Pair{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}

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
	count := 0
	bounds := Pair{len(grid), len(grid[0])}
	for i := range grid {
		for j := range grid[i] {
			if grid[i][j] == '0' {
				tails := make(map[Pair]struct{})
				stack := []Pair{{i, j}}
				for len(stack) > 0 {
					curr := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					if grid[curr.first][curr.second] == '9' {
						tails[curr] = struct{}{}
						continue
					}
					for _, d := range deltas {
						next := curr.Add(d)
						if inBounds(next, bounds) && grid[next.first][next.second] == grid[curr.first][curr.second]+1 {
							stack = append(stack, next)
						}
					}
				}
				count += len(tails)
			}
		}
	}
	return count
}

func part2(grid []string) int {
	var stack []Pair
	for i := range grid {
		for j := range grid[i] {
			if grid[i][j] == '0' {
				stack = append(stack, Pair{i, j})
			}
		}
	}

	count := 0
	bounds := Pair{len(grid), len(grid[0])}
	for len(stack) > 0 {
		curr := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		if grid[curr.first][curr.second] == '9' {
			count++
			continue
		}
		for _, d := range deltas {
			next := curr.Add(d)
			if inBounds(next, bounds) && grid[next.first][next.second] == grid[curr.first][curr.second]+1 {
				stack = append(stack, next)
			}
		}
	}
	return count
}
