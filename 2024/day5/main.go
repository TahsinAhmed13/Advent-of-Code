package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

type Rule struct {
	a, b int
}

func main() {
	reader := bufio.NewReader(os.Stdin)

	var rules []Rule
	for {
		line, _ := reader.ReadString('\n')
		line = strings.TrimSpace(line)
		if line == "" {
			break
		}
		parts := strings.Split(line, "|")
		a, _ := strconv.Atoi(parts[0])
		b, _ := strconv.Atoi(parts[1])
		rules = append(rules, Rule{a, b})
	}

	var updates [][]int
	for {
		line, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		line = strings.TrimSpace(line)
		var pages []int
		for _, s := range strings.Split(line, ",") {
			n, _ := strconv.Atoi(s)
			pages = append(pages, n)
		}
		updates = append(updates, pages)
	}

	fmt.Println("Part 1:", part1(rules, updates))
	fmt.Println("Part 2:", part2(rules, updates))
}

func part1(rules []Rule, updates [][]int) int {
	valid := make([]bool, len(updates))
	for i := range valid {
		valid[i] = true
	}
	for _, rule := range rules {
		for i, pages := range updates {
			if !slices.Contains(pages, rule.a) || !slices.Contains(pages, rule.b) {
				continue
			}
			seen := false
			for _, x := range pages {
				seen = seen || (x == rule.a)
				if x == rule.b {
					valid[i] = valid[i] && seen
				}
			}
		}
	}
	sum := 0
	for i, pages := range updates {
		if valid[i] {
			sum += pages[len(pages)/2]
		}
	}
	return sum
}

func part2(rules []Rule, updates [][]int) int {
	sum := 0
	for _, update := range updates {
		pos := make([]int, len(updates))
		for _, rule := range rules {
			if slices.Contains(update, rule.a) && slices.Contains(update, rule.b) {
				index := slices.Index(update, rule.b)
				pos[index]++
			}
		}
		sorted := true
		for i, p := range pos {
			sorted = sorted && p <= i
		}
		if !sorted {
			for i, p := range pos {
				if p == len(update)/2 {
					sum += update[i]
					break
				}
			}
		}
	}
	return sum
}
