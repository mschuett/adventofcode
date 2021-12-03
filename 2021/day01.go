package main

import (
	"fmt"
	"strconv"
	"strings"
)

func readNumbersAsSlice(inputText string) []int {
	var numbers []int
	for _, line := range strings.Split(strings.TrimSpace(inputText), "\n") {
		var num, _ = strconv.Atoi(strings.TrimSpace(line))
		numbers = append(numbers, num)
	}
	return numbers
}

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
