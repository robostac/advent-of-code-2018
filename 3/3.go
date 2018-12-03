package main

import (
	"bufio"
	"fmt"
	"os"
)

type point struct {
	x int
	y int
}

type claim struct {
	id int
	sx int
	sy int
	w  int
	h  int
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	m := map[point]int{}
	claims := []claim{}
	for scanner.Scan() {
		//#1 @ 1,3: 4x
		var c claim
		fmt.Sscanf(scanner.Text(), "#%d @ %d,%d: %dx%d", &c.id, &c.sx, &c.sy, &c.w, &c.h)
		for x := c.sx; x < c.sx+c.w; x++ {
			for y := c.sy; y < c.sy+c.h; y++ {
				p := point{x, y}
				m[p]++
			}
		}
		claims = append(claims, c)
	}
	count := 0
	for _, v := range m {
		if v > 1 {
			count++
		}
	}
	for _, c := range claims {
		valid := true
		for x := c.sx; x < c.sx+c.w; x++ {
			for y := c.sy; y < c.sy+c.h; y++ {
				p := point{x, y}
				if m[p] != 1 {
					valid = false
				}
			}
		}
		if valid {
			fmt.Println(c)
		}

	}
	fmt.Println(count)
}
