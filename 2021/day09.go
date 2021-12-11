package main

import (
	"fmt"
	"sort"
	"strings"
)

func day9() {
	// var inputText = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678"
	var inputText = downloadHelper(2021, 9)

	var height = make([][]int, 0)
	for _, line := range strings.Split(strings.TrimSpace(inputText), "\n") {
		inputLine := make([]int, len(line))
		for i := 0; i < len(line); i++ {
			inputLine[i] = int(line[i] - '0')
		}
		height = append(height, inputLine)
	}
	//for y := 0; y < len(height); y++ {
	//	fmt.Printf("%v\n", height[y])
	//}

	lowPointCoords := make([]Coordinate2d, 0)
	lowPointHeights := make([]int, 0)
	for y := 0; y < len(height); y++ {
		for x := 0; x < len(height[y]); x++ {
			if (y == 0 || height[y][x] < height[y-1][x]) &&
				(x == 0 || height[y][x] < height[y][x-1]) &&
				(y+1 == len(height) || height[y][x] < height[y+1][x]) &&
				(x+1 == len(height[y]) || height[y][x] < height[y][x+1]) {
				lowPointCoords = append(lowPointCoords, Coordinate2d{x, y})
				lowPointHeights = append(lowPointHeights, height[y][x])
			}
		}
	}
	risk := 0
	for i := 0; i < len(lowPointHeights); i++ {
		lp := lowPointHeights[i]
		risk += lp + 1
	}
	fmt.Printf("risk %d from lowpoints: %v at %v\n", risk, lowPointHeights, lowPointCoords)

	// Part 2 - basins
	var basinSize = make([]int, len(lowPointHeights))
	for lpNum, lpCoord := range lowPointCoords {
		var basinBitmap = make([][]bool, len(height))
		for i := 0; i < len(height); i++ {
			basinBitmap[i] = make([]bool, len(height[0]))
		}
		basinBitmap[lpCoord.y][lpCoord.x] = true
		// now try to grow the basin, from points base_xy to target_xy
		basinGrowth := false
		for true {
			for by := 0; by < len(height); by++ {
				for bx := 0; bx < len(height[by]); bx++ {
					// find first point of basin
					if !basinBitmap[by][bx] {
						continue
					}
					// to left
					for tx := bx - 1; tx >= 0; tx-- {
						if basinBitmap[by][tx] || height[by][tx] == 9 {
							break // already basin, or 9
						} else {
							basinBitmap[by][tx] = true
							basinGrowth = true
						}
					}
					// to right
					for tx := bx + 1; tx < len(height[by]); tx++ {
						if basinBitmap[by][tx] || height[by][tx] == 9 {
							break // already basin, or 9
						} else {
							basinBitmap[by][tx] = true
							basinGrowth = true
						}
					}
					// to top
					for ty := by - 1; ty >= 0; ty-- {
						if basinBitmap[ty][bx] || height[ty][bx] == 9 {
							break // already basin, or 9
						} else {
							basinBitmap[ty][bx] = true
							basinGrowth = true
						}
					}
					// to bottom
					for ty := by + 1; ty < len(height); ty++ {
						if basinBitmap[ty][bx] || height[ty][bx] == 9 {
							break // already basin, or 9
						} else {
							basinBitmap[ty][bx] = true
							basinGrowth = true
						}
					}
				}
			}
			if !basinGrowth {
				break
			}
			basinGrowth = false // reset
		}

		for y := 0; y < len(height); y++ {
			for x := 0; x < len(height[y]); x++ {
				if basinBitmap[y][x] {
					basinSize[lpNum]++
				}
			}
		}
		fmt.Printf("basin %d, size %d\n", lpNum, basinSize[lpNum])
		//for i := 0; i < len(height); i++ {
		//	fmt.Printf("%v\n", basinBitmap[i])
		//}
	}
	sort.Ints(basinSize)
	result := 1
	for i := len(basinSize) - 1; i >= len(basinSize)-3; i-- {
		result *= basinSize[i]
	}
	fmt.Printf("final result: %d\n", result)
}
