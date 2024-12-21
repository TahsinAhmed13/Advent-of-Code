package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

var dx = []int{1, 1, 0, -1, -1, -1, 0, 1}
var dy = []int{0, 1, 1, 1, 0, -1, -1, -1}

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

	fmt.Println("Part 1:", part1(grid, "XMAS"))
	fmt.Println("Part 2:", part2(grid, "MAS"))
}

func part1(grid []string, keyword string) int {
	count := 0
	for i := range grid {
		for j := range grid[i] {
			for k := 0; k < len(dx); k++ {
				found := true
				for l := 0; l < len(keyword); l++ {
					x := i + dx[k]*l
					y := j + dy[k]*l
					if x < 0 || x >= len(grid) || y < 0 || y >= len(grid[x]) || grid[x][y] != keyword[l] {
						found = false
						break
					}
				}
				if found {
					count++
					continue
				}
			}
		}
	}
	return count
}

func part2(grid []string, keyword string) int {
	count := 0
	for i := 0; i <= len(grid)-len(keyword); i++ {
		for j := 0; j <= len(grid[i])-len(keyword); j++ {
			diagM := true
			for k := 0; k < len(keyword); k++ {
				if grid[i+k][j+k] != keyword[k] {
					diagM = false
					break
				}
			}
			if !diagM {
				diagM = true
				for k := 0; k < len(keyword); k++ {
					if grid[i+len(keyword)-1-k][j+len(keyword)-1-k] != keyword[k] {
						diagM = false
						break
					}
				}
			}

			diagR := true
			for k := 0; k < len(keyword); k++ {
				if grid[i+k][j+len(keyword)-1-k] != keyword[k] {
					diagR = false
					break
				}
			}
			if !diagR {
				diagR = true
				for k := 0; k < len(keyword); k++ {
					if grid[i+len(keyword)-1-k][j+k] != keyword[k] {
						diagR = false
						break
					}
				}
			}

			if diagM && diagR {
				count++
			}
		}
	}
	return count
}
