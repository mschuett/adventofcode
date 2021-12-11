package main

import (
	"fmt"
	"strings"
)

func octoFlashPrint(pocto *[][]int, pflashBitmap *[][]bool) {
	octo := *pocto
	flashBitmap := *pflashBitmap
	var Reset = "\033[0m"
	var White = "\033[97m"

	for y := 0; y < len(octo); y++ {
		for x := 0; x < len(octo[y]); x++ {
			pre, post := "", ""
			if flashBitmap[y][x] {
				pre, post = White, Reset
			}
			fmt.Printf("%s%1X%s", pre, octo[y][x], post)
		}
		fmt.Printf("\n")
	}
}
func octoStep(pocto *[][]int, doPrint bool) int {
	octo := *pocto

	flashBitmap := make([][]bool, len(octo))
	for i := 0; i < len(octo); i++ {
		flashBitmap[i] = make([]bool, len(octo[0]))
	}
	for y := 0; y < len(octo); y++ {
		for x := 0; x < len(octo[y]); x++ {
			flashBitmap[y][x] = false
		}
	}
	for y := 0; y < len(octo); y++ {
		for x := 0; x < len(octo[y]); x++ {
			octo[y][x]++
		}
	}

	for {
		updated := false
		for y := 0; y < len(octo); y++ {
			for x := 0; x < len(octo[y]); x++ {
				if flashBitmap[y][x] == false && octo[y][x] > 9 {
					flashBitmap[y][x] = true
					updated = true

					if (y > 0) && (x > 0) {
						octo[y-1][x-1]++
					}
					if y > 0 {
						octo[y-1][x]++
					}
					if (y > 0) && (x < len(octo[y])-1) {
						octo[y-1][x+1]++
					}

					if x > 0 {
						octo[y][x-1]++
					}
					if x < len(octo[y])-1 {
						octo[y][x+1]++
					}

					if (y < len(octo)-1) && (x > 0) {
						octo[y+1][x-1]++
					}
					if y < len(octo)-1 {
						octo[y+1][x]++
					}
					if (y < len(octo)-1) && (x < len(octo[y])-1) {
						octo[y+1][x+1]++
					}
				}
			}
		}
		if !updated {
			break
		}
	}

	sumOfFlashes := 0
	for y := 0; y < len(octo); y++ {
		for x := 0; x < len(octo[y]); x++ {
			if flashBitmap[y][x] {
				octo[y][x] = 0
				sumOfFlashes++
			}
		}
	}
	for y := 0; y < len(octo); y++ {
		for x := 0; x < len(octo[y]); x++ {
			if octo[y][x] > 9 {
				fmt.Printf("Error, point (%d,%d) has value %d > 9\n", x, y, octo[y][x])
			}
		}
	}
	if doPrint {
		octoFlashPrint(&octo, &flashBitmap)
	}
	return sumOfFlashes
}

func day11() {
	//var inputText = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526\n"
	var inputText = downloadHelper(2021, 11)

	var octoEnergy = make([][]int, 0)
	for _, line := range strings.Split(strings.TrimSpace(inputText), "\n") {
		inputLine := make([]int, len(line))
		for i := 0; i < len(line); i++ {
			inputLine[i] = int(line[i] - '0')
		}
		octoEnergy = append(octoEnergy, inputLine)
	}

	sumOfFlashes := 0
	for i := 1; true; i++ {
		flashes := octoStep(&octoEnergy, false)
		sumOfFlashes += flashes
		fmt.Printf("step %d: %d flashes (%d total)\n", i, flashes, sumOfFlashes)

		if flashes == len(octoEnergy)*len(octoEnergy[0]) {
			fmt.Printf("all octopuses flashed simultaneously!\n")
			break
		}
	}
}
