package main

import (
	"fmt"
	"strconv"
	"strings"
)

const maxDimension int = 1000

type Coordinate2d struct {
	x int
	y int
}
type VentLine2d struct {
	from Coordinate2d
	to   Coordinate2d
}
type Map2d [maxDimension][maxDimension]int

func minmaxCoordinates(a, b Coordinate2d) (Coordinate2d, Coordinate2d) {
	if a.x+a.y < b.x+b.y {
		return a, b
	}
	return b, a
}

func readVentLinesCoordinates(inputText string) []VentLine2d {
	var lines = strings.Split(strings.TrimSpace(inputText), "\n")
	var ventLines = make([]VentLine2d, len(lines))

	for i := 0; i < len(lines); i++ {
		textPair := strings.Split(lines[i], " -> ")
		leftPosText := textPair[0]
		rightPosText := textPair[1]

		leftPosPair := strings.Split(leftPosText, ",")
		leftPosX, _ := strconv.Atoi(leftPosPair[0])
		leftPosY, _ := strconv.Atoi(leftPosPair[1])
		leftPos := Coordinate2d{leftPosX, leftPosY}

		rightPosPair := strings.Split(rightPosText, ",")
		rightPosX, _ := strconv.Atoi(rightPosPair[0])
		rightPosY, _ := strconv.Atoi(rightPosPair[1])
		rightPos := Coordinate2d{rightPosX, rightPosY}

		ventLines[i] = VentLine2d{leftPos, rightPos}
	}
	return ventLines
}

func createMapWithHorizontalVentLines(ventLines []VentLine2d) Map2d {
	mymap := Map2d{}
	for i := 0; i < len(ventLines); i++ {
		from, to := minmaxCoordinates(ventLines[i].from, ventLines[i].to)
		if from.x == to.x {
			for y := from.y; y <= to.y; y++ {
				mymap[from.x][y] += 1
			}
		} else if from.y == to.y {
			for x := from.x; x <= to.x; x++ {
				mymap[x][from.y] += 1
			}
		} else {
			// ignore non-horizontal line
			fmt.Printf("ignoring %d,%d -> %d,%d\n",
				ventLines[i].from.x, ventLines[i].from.y,
				ventLines[i].to.x, ventLines[i].to.y)
		}
	}
	return mymap
}

func createMapWithVentLines(ventLines []VentLine2d) Map2d {
	mymap := Map2d{}
	for i := 0; i < len(ventLines); i++ {
		from, to := minmaxCoordinates(ventLines[i].from, ventLines[i].to)
		if from.x == to.x {
			// vertical
			for y := from.y; y <= to.y; y++ {
				mymap[from.x][y] += 1
			}
		} else if from.y == to.y {
			// horizontal
			for x := from.x; x <= to.x; x++ {
				mymap[x][from.y] += 1
			}
		} else if (to.y - from.y) == (to.x - from.x) {
			// diagonal left-to-right
			for i := 0; i <= (to.x - from.x); i++ {
				mymap[from.x+i][from.y+i] += 1
			}
		} else if (from.y - to.y) == (to.x - from.x) {
			// diagonal right-to-left
			// still two cases, depending on x/y dir
			for i := 0; i <= (to.x - from.x); i++ {
				mymap[from.x+i][from.y-i] += 1
			}
			for i := 0; i <= (from.x - to.x); i++ {
				mymap[from.x-i][from.y+i] += 1
			}
		} else {
			fmt.Printf("unhandled line %d,%d -> %d,%d\n",
				ventLines[i].from.x, ventLines[i].from.y,
				ventLines[i].to.x, ventLines[i].to.y)
		}
	}
	return mymap
}

func printMap(mymap Map2d) {
	for y := 0; y < maxDimension; y++ {
		for x := 0; x < maxDimension; x++ {
			if mymap[x][y] == 0 {
				fmt.Printf(".")
			} else {
				fmt.Printf("%d", mymap[x][y])
			}
		}
		fmt.Printf("\n")
	}
}

func countOverlappingLinesCoordinates(mymap Map2d) int {
	count := 0
	for y := 0; y < maxDimension; y++ {
		for x := 0; x < maxDimension; x++ {
			if mymap[x][y] > 1 {
				count++
			}
		}
	}
	return count
}

func day5() {
	var inputText = downloadHelper(2021, 5)
	// var inputText = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2\n"
	ventLines := readVentLinesCoordinates(inputText)
	//for i := 0; i < len(ventLines); i++ {
	//	fmt.Printf("%d,%d -> %d,%d\n",
	//		ventLines[i].from.x, ventLines[i].from.y,
	//		ventLines[i].to.x, ventLines[i].to.y)
	//}
	mymap := createMapWithVentLines(ventLines)
	// printMap(mymap)
	overlappingCoordinates := countOverlappingLinesCoordinates(mymap)
	fmt.Printf("count: %v\n", overlappingCoordinates)
}
