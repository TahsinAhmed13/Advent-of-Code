package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	reader := bufio.NewReader(os.Stdin)

	input, _ := reader.ReadString('\n')

	fmt.Println("Part 1:", part1(input))
	fmt.Println("Part 2:", part2(input))
}

func part1(input string) int {
	l, r := 0, 2*((len(input)-1)/2)
	pos, cnt := 0, 0
	checksum := 0
	for l < r {
		if l%2 == 0 {
			for i := 0; i < int(input[l]-'0'); i++ {
				checksum += pos * (l / 2)
				pos++
			}
		} else {
			for i := 0; i < int(input[l]-'0') && l < r; {
				for i < int(input[l]-'0') && cnt < int(input[r]-'0') {
					checksum += pos * (r / 2)
					pos++
					cnt++
					i++
				}
				if cnt >= int(input[r]-'0') {
					r -= 2
					cnt = 0
				}
			}
		}
		l++
	}
	if l == r {
		for cnt < int(input[l]-'0') {
			checksum += pos * (l / 2)
			pos++
			cnt++
		}
	}
	return checksum
}

func part2(input string) int {
	presum := make([]int, len(input)+1)
	for i := 0; i < len(input); i++ {
		presum[i+1] = presum[i] + int(input[i]-'0')
	}

	checksum := 0
	spaces := make([]int, len(input))
	for i := 0; i < len(input); i++ {
		if i%2 == 0 {
			for j := 0; j < int(input[i]-'0'); j++ {
				checksum += (presum[i] + j) * (i / 2)
			}
		} else {
			spaces[i] = int(input[i] - '0')
		}
	}

	for i := 2*(len(input)-1)/2 - 1; i >= 0; i -= 2 {
		for j := 1; j < i; j += 2 {
			if spaces[j] >= int(input[i]-'0') {
				for k := 0; k < int(input[i]-'0'); k++ {
					checksum += (presum[j+1] - spaces[j]) * (i / 2)
					checksum -= (presum[i] + k) * (i / 2)
					spaces[j]--
				}
				break
			}
		}
	}
	return checksum
}
