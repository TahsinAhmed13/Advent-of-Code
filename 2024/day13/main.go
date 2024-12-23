package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type Pair struct {
	first, second int
}

type Claw struct {
	abtn, bbtn, dest Pair
}

func main() {
	reader := bufio.NewReader(os.Stdin)

	var claws []Claw
	for {
		aline, _ := reader.ReadString('\n')
		bline, _ := reader.ReadString('\n')
		dline, _ := reader.ReadString('\n')

		afields := strings.Fields(strings.TrimSpace(aline))
		ax, _ := strconv.Atoi(afields[2][2 : len(afields[2])-1])
		ay, _ := strconv.Atoi(afields[3][2:])

		bfields := strings.Fields(strings.TrimSpace(bline))
		bx, _ := strconv.Atoi(bfields[2][2 : len(bfields[2])-1])
		by, _ := strconv.Atoi(bfields[3][2:])

		dfields := strings.Fields(strings.TrimSpace(dline))
		dx, _ := strconv.Atoi(dfields[1][2 : len(dfields[1])-1])
		dy, _ := strconv.Atoi(dfields[2][2:])

		claws = append(claws, Claw{Pair{ax, ay}, Pair{bx, by}, Pair{dx, dy}})

		if _, err := reader.ReadString('\n'); err != nil {
			break
		}
	}

	fmt.Println("Part 1:", part1(claws))
	fmt.Println("Part 2:", part2(claws))
}

func part1(claws []Claw) int {
	tokens := 0
	for _, c := range claws {
		t := math.MaxInt
		for i := 0; i < 100; i++ {
			dx, dy := c.dest.first-c.abtn.first*i, c.dest.second-c.abtn.second*i
			if dx < 0 || dy < 0 {
				break
			}
			if dx%c.bbtn.first == 0 && dy%c.bbtn.second == 0 {
				j, k := dx/c.bbtn.first, dy/c.bbtn.second
				if j == k {
					t = min(t, 3*i+j)
				}
			}
		}
		if t != math.MaxInt {
			tokens += t
		}
	}
	return tokens
}

func gcd(a int, b int, x *int, y *int) int {
	if a < b {
		return gcd(b, a, y, x)
	}
	if b == 0 {
		*x = 1
		*y = 0
		return a
	}
	var x1, y1 int
	d := gcd(b, a%b, &x1, &y1)
	*x = y1
	*y = x1 - (a/b)*y1
	return d
}

func det(a int, b int, c int, d int) int {
	return a*d - b*c
}

func cramer(a1 int, b1 int, c1 int, a2 int, b2 int, c2 int) (int, int) {
	d := det(a1, b1, a2, b2)
	dx := det(c1, b1, c2, b2)
	dy := det(a1, c1, a2, c2)
	return -dx / d, -dy / d
}

func part2(claws []Claw) int {
	const offset = 10000000000000

	tokens := 0
	for _, c := range claws {
		var p01, q01, p02, q02 int
		dest := Pair{c.dest.first + offset, c.dest.second + offset}
		g1 := gcd(c.abtn.first, c.bbtn.first, &p01, &q01)
		g2 := gcd(c.abtn.second, c.bbtn.second, &p02, &q02)
		if dest.first%g1 != 0 || dest.second%g2 != 0 {
			continue
		}
		p01 *= dest.first / g1
		q01 *= dest.first / g1
		p02 *= dest.second / g2
		q02 *= dest.second / g2

		a1, b1, c1 := c.abtn.first, c.bbtn.first, -(c.abtn.first*p01 + c.bbtn.first*q01)
		a2, b2, c2 := c.abtn.second, c.bbtn.second, -(c.abtn.second*p02 + c.bbtn.second*q02)
		x, y := cramer(a1, b1, c1, a2, b2, c2)
		if x < 0 || y < 0 {
			continue
		}
		if c.abtn.first*x+c.bbtn.first*y != dest.first || c.abtn.second*x+c.bbtn.second*y != dest.second {
			continue
		}
		tokens += 3*x + y
	}
	return tokens
}
