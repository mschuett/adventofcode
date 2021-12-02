package main

import (
	"fmt"
	"strconv"
	"strings"
)

func day1aImpl(inputText string) int {
	var numbers = readNumbersAsSlice(inputText)
	// fmt.Printf("%v\n", numbers)

	var increases = 0
	for i := 1; i < len(numbers); i++ {
		if numbers[i] > numbers[i-1] {
			increases++
		}
	}
	return increases
}

func day1bImpl(inputText string) int {
	var numbers = readNumbersAsSlice(inputText)
	var windowSums []int
	for i := 2; i < len(numbers); i++ {
		windowSums = append(windowSums, numbers[i]+numbers[i-1]+numbers[i-2])
	}

	var increases = 0
	for i := 1; i < len(windowSums); i++ {
		if windowSums[i] > windowSums[i-1] {
			increases++
		}
	}
	return increases
}

func day1() {
	var inputText = downloadHelper(2021, 1)
	fmt.Printf("%d increases in measurements\n", day1aImpl(inputText))
	fmt.Printf("%d increases in windowed measurements\n", day1bImpl(inputText))
}

type Direction string

const (
	Up      Direction = "up"
	Down              = "down"
	Forward           = "forward"
)

type DirInstruction struct {
	dir    Direction
	amount int
}

type Position struct {
	depth   int
	forward int
}

type AimedPosition struct {
	depth   int
	forward int
	aim     int
}

func readDirections(inputText string) []DirInstruction {
	var directions []DirInstruction
	for _, line := range strings.Split(strings.TrimSpace(inputText), "\n") {
		var lineContent = strings.Split(strings.TrimSpace(line), " ")
		if (len(lineContent)) != 2 { // skip unexpected format
			continue
		}
		var num, _ = strconv.Atoi(strings.TrimSpace(lineContent[1]))
		var tmpDir = DirInstruction{dir: Direction(lineContent[0]), amount: num}
		directions = append(directions, tmpDir)
	}
	return directions
}

func day2aImpl(inputText string) int {

	var instructions = readDirections(inputText)
	fmt.Printf("%v\n", instructions)

	var pos = Position{0, 0}
	for _, instruction := range instructions {
		switch instruction.dir {
		case Forward:
			pos.forward += instruction.amount
		case Up:
			pos.depth -= instruction.amount
		case Down:
			pos.depth += instruction.amount
		}
	}

	var mult = pos.depth * pos.forward
	fmt.Printf("final position: %v, multiplied: %v\n", pos, mult)
	return mult
}

func day2bImpl(inputText string) int {
	var instructions = readDirections(inputText)
	fmt.Printf("%v\n", instructions)

	var pos = AimedPosition{0, 0, 0}
	for _, instruction := range instructions {
		switch instruction.dir {
		case Forward:
			pos.forward += instruction.amount
			pos.depth += instruction.amount * pos.aim
		case Up:
			pos.aim -= instruction.amount
		case Down:
			pos.aim += instruction.amount
		}
	}

	var mult = pos.depth * pos.forward
	fmt.Printf("final position: %v, multiplied: %v\n", pos, mult)
	return mult
}

func day2() {
	var inputText = downloadHelper(2021, 2)
	fmt.Printf("%v\n", day2bImpl(inputText))
}

func main() {
	day2()
}
