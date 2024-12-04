package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	reader := bufio.NewReader(os.Stdin)

	var report [][]int
	for {
		line, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		var row []int
		for _, s := range strings.Fields(line) {
			x, _ := strconv.Atoi(s)
			row = append(row, x)
		}
		report = append(report, row)
	}

	fmt.Println("Part 1:", part1(report))
	fmt.Println("Part 2:", part2(report))
}

func is_safe(a []int) bool {
	safe := true
	for i := 0; i < len(a)-1; i++ {
		diff := a[i+1] - a[i]
		safe = safe && (diff >= 1 && diff <= 3)
	}
	if safe {
		return true
	}
	safe = true
	for i := len(a) - 1; i > 0; i-- {
		diff := a[i-1] - a[i]
		safe = safe && (diff >= 1 && diff <= 3)
	}
	return safe
}

func part1(report [][]int) int {
	count := 0
	for _, row := range report {
		if is_safe(row) {
			count += 1
		}
	}
	return count
}

func part2(report [][]int) int {
	count := 0
	for _, row := range report {
		for i := 0; i < len(row); i++ {
			r := make([]int, len(row)-1)
			for j := 0; j < i; j++ {
				r[j] = row[j]
			}
			for j := i + 1; j < len(row); j++ {
				r[j-1] = row[j]
			}
			if is_safe(r) {
				count += 1
				break
			}
		}
	}
	return count
}
