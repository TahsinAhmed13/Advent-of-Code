package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Node struct {
	value int
	next  *Node
}

func main() {
	reader := bufio.NewReader(os.Stdin)

	input, _ := reader.ReadString('\n')
	input = strings.TrimSpace(input)

	head := &Node{}
	curr := head
	for _, n := range strings.Fields(input) {
		value, _ := strconv.Atoi(n)
		curr.next = &Node{value, nil}
		curr = curr.next
	}

	fmt.Println("Part 1:", part1(head.next))
}

func printList(head *Node) {
	curr := head
	for curr != nil {
		fmt.Print(curr.value, " ")
		curr = curr.next
	}
	fmt.Println()
}

func part1(head *Node) int {
	for i := 0; i < 25; i++ {
		curr := head
		for curr != nil {
			s := fmt.Sprintf("%d", curr.value)
			if curr.value == 0 {
				curr.value = 1
			} else if len(s)%2 == 0 {
				left, _ := strconv.Atoi(s[:len(s)/2])
				right, _ := strconv.Atoi(s[len(s)/2:])
				curr.value = left
				curr.next = &Node{right, curr.next}
				curr = curr.next
			} else {
				curr.value *= 2024
			}
			curr = curr.next
		}
	}

	len := 0
	curr := head
	for curr != nil {
		len++
		curr = curr.next
	}
	return len
}
