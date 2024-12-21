package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strings"
)

type Pair struct {
	first, second int
}

func (p *Pair) Add(p2 Pair) Pair {
	return Pair{p.first + p2.first, p.second + p2.second}
}

func (p *Pair) Sub(p2 Pair) Pair {
	return Pair{p.first - p2.first, p.second - p2.second}
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

func inBounds(pos Pair, bounds Pair) bool {
	return pos.first >= 0 && pos.first < bounds.first && pos.second >= 0 && pos.second < bounds.second
}

func part1(grid []string) int {
	antennas := make(map[byte][]Pair)
	for i := range grid {
		for j := range grid[i] {
			if grid[i][j] != '.' {
				antennas[grid[i][j]] = append(antennas[grid[i][j]], Pair{i, j})
			}
		}
	}

	bounds := Pair{len(grid), len(grid[0])}
	antinodes := make(map[Pair]struct{})
	for _, pos := range antennas {
		for i := 0; i < len(pos); i++ {
			for j := i + 1; j < len(pos); j++ {
				dy := int(math.Abs(float64(pos[i].first - pos[j].first)))
				dx := int(math.Abs(float64(pos[i].second - pos[j].second)))

				if pos[i].first > pos[j].first {
					dy *= -1
				}
				if pos[i].second > pos[j].second {
					dx *= -1
				}
				delta := Pair{dy, dx}

				p1 := pos[i].Sub(delta)
				if inBounds(p1, bounds) {
					antinodes[p1] = struct{}{}
				}

				p2 := pos[j].Add(delta)
				if inBounds(p2, bounds) {
					antinodes[p2] = struct{}{}
				}
			}
		}
	}
	return len(antinodes)
}

func part2(grid []string) int {
	antennas := make(map[byte][]Pair)
	for i := range grid {
		for j := range grid[i] {
			if grid[i][j] != '.' {
				antennas[grid[i][j]] = append(antennas[grid[i][j]], Pair{i, j})
			}
		}
	}

	bounds := Pair{len(grid), len(grid[0])}
	antinodes := make(map[Pair]struct{})
	for _, pos := range antennas {
		for i := 0; i < len(pos); i++ {
			for j := i + 1; j < len(pos); j++ {
				dy := int(math.Abs(float64(pos[i].first - pos[j].first)))
				dx := int(math.Abs(float64(pos[i].second - pos[j].second)))

				if pos[i].first > pos[j].first {
					dy *= -1
				}
				if pos[i].second > pos[j].second {
					dx *= -1
				}
				delta := Pair{dy, dx}

				currLeft := pos[i]
				for inBounds(currLeft, bounds) {
					antinodes[currLeft] = struct{}{}
					currLeft = currLeft.Sub(delta)
				}

				currRight := pos[j]
				for inBounds(currRight, bounds) {
					antinodes[currRight] = struct{}{}
					currRight = currRight.Add(delta)
				}
			}
		}
	}
	return len(antinodes)
}
