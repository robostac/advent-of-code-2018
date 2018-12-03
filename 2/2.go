package main

import (
	"bufio"
	"fmt"
	"os"
)

func val(s string) (int, int) {
	test := [26]int{}
	for _, v := range s {
		test[v-'a']++
	}

	twos := 0
	threes := 0
	for _, v := range test {
		if v == 2 {
			twos = 1
		}
		if v == 3 {
			threes = 1
		}
	}
	return twos, threes
}

func diff(x string, y string) (string, string) {
	same := ""
	diff := ""
	for i, v := range x {
		vv := rune(y[i])
		if vv == v {
			same += string(vv)
		} else {
			diff += string(vv)
		}
	}
	return same, diff
}

func main() {
	fmt.Println(val("bababc"))
	scanner := bufio.NewScanner(os.Stdin)
	freq := []string{}
	twos := 0
	threes := 0
	for scanner.Scan() {
		var x string
		fmt.Sscan(scanner.Text(), &x)
		freq = append(freq, x)
		t, tt := val(x)
		twos += t
		threes += tt
	}
	for i, v := range freq {
		for _, vv := range freq[i+1:] {
			s, d := diff(v, vv)
			if len(d) == 1 {
				fmt.Println(s)
			}

		}
	}
	fmt.Println(twos * threes)
}
