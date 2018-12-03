package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {

	scanner := bufio.NewScanner(os.Stdin)
	fmt.Println("hello")
	v := 0
	freq := []int{}
	for scanner.Scan() {
		var x int
		fmt.Sscan(scanner.Text(), &x)
		v += x
		freq = append(freq, x)
	}
	v2 := 0
	seen := map[int]struct{}{}
	offset := 0
	for true {
		v2 += freq[offset]
		_, ok := seen[v2]
		if ok {
			break
		}

		offset++
		if offset == len(freq) {
			offset = 0
		}
		seen[v2] = struct{}{}
	}
	fmt.Println(v, v2)
}
