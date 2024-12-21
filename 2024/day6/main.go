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

type Position struct {
	coords    Pair
	direction Pair
}

func (p *Pair) add(q Pair) Pair {
	return Pair{p.first + q.first, p.second + q.second}
}

func (p *Pair) rot90() Pair {
	return Pair{p.second, -p.first}
}

func (p *Position) advance(grid []string) bool {
	next := p.coords.add(p.direction)
	inBounds := next.first >= 0 && next.first < len(grid) && next.second >= 0 && next.second < len(grid[next.first])
	if !inBounds {
		return false
	}
	if grid[next.first][next.second] == '#' {
		p.direction = p.direction.rot90()
	}
	p.coords = p.coords.add(p.direction)
	return true
}

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

func findCoords(grid []string, ch string) Pair {
	for i := range grid {
		j := strings.Index(grid[i], ch)
		if j != -1 {
			return Pair{i, j}
		}
	}
	return Pair{-1, -1}
}

func part1(grid []string) int {
	vis := make(map[Pair]struct{})
	start := findCoords(grid, "^")
	pos := Position{start, Pair{-1, 0}}
	for {
		vis[pos.coords] = struct{}{}
		if !pos.advance(grid) {
			break
		}
	}
	return len(vis)
}

func hasLoop(grid []string) bool {
	vis := make(map[Position]struct{})
	start := findCoords(grid, "^")
	pos := Position{start, Pair{-1, 0}}
	for {
		if _, exists := vis[pos]; exists {
			return true
		}
		vis[pos] = struct{}{}
		if !pos.advance(grid) {
			break
		}
	}
	return false
}

func part2(grid []string) int {
	count := 0
	for i := range grid {
		row := []rune(grid[i])
		for j := range grid[i] {
			if grid[i][j] == '.' {
				row[j] = '#'
				grid[i] = string(row)
				if hasLoop(grid) {
					count++
				}
				row[j] = '.'
				grid[i] = string(row)
			}
		}
	}
	return count
}
