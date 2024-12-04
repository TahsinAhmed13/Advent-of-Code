package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	reader := bufio.NewReader(os.Stdin)

	var a, b []int
	for {
		line, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		nums := strings.Fields(line)
		if len(nums) < 2 {
			continue
		}
		x, _ := strconv.Atoi(nums[0])
		y, _ := strconv.Atoi(nums[1])
		a = append(a, x)
		b = append(b, y)
	}

	fmt.Println("Part 1:", part1(a, b))
	fmt.Println("Part 2:", part2(a, b))
}

func part1(a, b []int) int {
	slices.Sort(a)
	slices.Sort(b)

	sum := 0
	for i := 0; i < len(a); i++ {
		sum += int(math.Abs(float64(a[i] - b[i])))
	}
	return sum
}

func part2(a, b []int) int {
	score := 0
	for _, x := range a {
		for _, y := range b {
			if x == y {
				score += x
			}
		}
	}
	return score
}
