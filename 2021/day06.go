package main

import (
	"fmt"
	"strconv"
	"strings"
)

func readCommaNumbersAsSlice(inputText string) []int {
	var numbers []int
	for _, line := range strings.Split(strings.TrimSpace(inputText), ",") {
		var num, _ = strconv.Atoi(strings.TrimSpace(line))
		numbers = append(numbers, num)
	}
	return numbers
}

func printFishBucketState(bucket map[int]int, day int) string {
	count := 0
	for _, j := range bucket {
		count += j
	}
	return fmt.Sprintf("Day %2d, %d fish in buckets %v", day, count, bucket)
}

func day6() {
	//var inputText = "3,4,3,1,2"
	var inputText = downloadHelper(2021, 6)
	var numbers []int = readCommaNumbersAsSlice(inputText)

	// Part 1 with "simple" simulation per fish
	var simulationDays = 80
	var fishes = make([]int, 0)
	for _, num := range numbers {
		fishes = append(fishes, num)
	}
	fmt.Printf("Initial State: %v\n", fishes)
	for day := 1; day <= simulationDays; day++ {
		for i, fish := range fishes {
			if fish == 0 {
				fishes[i] = 7 // will be decreased
				fishes = append(fishes, 9)
			}
			fishes[i]--
		}
		fmt.Printf("After % 2d days: %d fishes\n", day, len(fishes))
	}

	// Part 2, refactored
	var fishBuckets = make(map[int]int)
	for _, num := range numbers {
		fishBuckets[num]++
	}
	println(printFishBucketState(fishBuckets, 0))

	simulationDays = 256
	for day := 1; day <= simulationDays; day++ {
		var newBuckets = make(map[int]int)
		// count timer
		for timer := 1; timer <= 8; timer++ {
			newBuckets[timer-1] = fishBuckets[timer]
		}
		newBuckets[8] = fishBuckets[0]  // new fishes
		newBuckets[6] += fishBuckets[0] // resets

		fishBuckets = newBuckets
		println(printFishBucketState(fishBuckets, day))
	}
}
