package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Equation struct {
	target   int
	operands []int
}

func main() {
	reader := bufio.NewReader(os.Stdin)

	var equations []Equation
	for {
		line, err := reader.ReadString('\n')
		if err != nil {
			break
		}

		parts := strings.Fields(line)
		target, _ := strconv.Atoi(parts[0][:len(parts[0])-1])
		var operands []int
		for _, part := range parts[1:] {
			operand, _ := strconv.Atoi(part)
			operands = append(operands, operand)
		}
		equations = append(equations, Equation{target, operands})
	}

	fmt.Println("Part 1:", part1(equations))
	fmt.Println("Part 2:", part2(equations))
}

func part1(equations []Equation) int {
	sum := 0
	for _, equation := range equations {
		for i := 0; i < (1 << (len(equation.operands) - 1)); i++ {
			val := equation.operands[0]
			for j := 0; j < len(equation.operands)-1; j++ {
				if i&(1<<j) != 0 {
					val += equation.operands[j+1]
				} else {
					val *= equation.operands[j+1]
				}
			}
			if val == equation.target {
				sum += equation.target
				break
			}
		}
	}
	return sum
}

func part2(equations []Equation) int {
	maxOperands := 0
	for _, equations := range equations {
		maxOperands = max(maxOperands, len(equations.operands))
	}
	pow3 := make([]int, maxOperands)
	pow3[0] = 1
	for i := 1; i < maxOperands; i++ {
		pow3[i] = pow3[i-1] * 3
	}

	sum := 0
	for _, equation := range equations {
		for i := 0; i < pow3[len(equation.operands)-1]; i++ {
			val := equation.operands[0]
			for j := 0; j < len(equation.operands)-1; j++ {
				switch i / pow3[j] % 3 {
				case 0:
					val += equation.operands[j+1]
				case 1:
					val *= equation.operands[j+1]
				case 2:
					a := strconv.Itoa(val)
					b := strconv.Itoa(equation.operands[j+1])
					val, _ = strconv.Atoi(a + b)
				}
			}
			if val == equation.target {
				sum += equation.target
				break
			}
		}
	}
	return sum
}
