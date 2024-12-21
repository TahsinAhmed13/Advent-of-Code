package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"unicode"
)

const (
	doop   = "do()"
	dontop = "don't()"
)

func main() {
	reader := bufio.NewReader(os.Stdin)

	var input string
	for {
		line, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		input += line
	}

	fmt.Println("Part 1:", part1(input))
	fmt.Println("Part 2:", part2(input))
}

func parse_mul(input string) (int, int) {
	if len(input) >= 6 && input[:4] == "mul(" {
		a, b := 0, 0
		j := 4
		for j < len(input) && unicode.IsDigit(rune(input[j])) {
			a = a*10 + int(input[j]-'0')
			j++
		}
		if input[j] != ',' {
			return 0, 1
		}
		j++
		for j < len(input) && unicode.IsDigit(rune(input[j])) {
			b = b*10 + int(input[j]-'0')
			j++
		}
		if input[j] != ')' {
			return 0, 1
		}
		return a * b, j
	}
	return 0, 1
}

func part1(input string) int {
	sum := 0
	for i := 0; i < len(input); {
		s, j := parse_mul(input[i:])
		sum += s
		i += j
	}
	return sum
}

func part2(input string) int {
	sum := 0
	flag := true
	for i := 0; i < len(input); {
		if strings.HasPrefix(input[i:], doop) {
			flag = true
			i += len(doop)
		} else if strings.HasPrefix(input[i:], dontop) {
			flag = false
			i += len(dontop)
		} else {
			s, j := parse_mul(input[i:])
			if flag {
				sum += s
			}
			i += j
		}
	}
	return sum
}
