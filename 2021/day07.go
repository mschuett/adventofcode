package main

import "fmt"

// Abs returns the absolute value of x.
func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func crabFuelUsage(dist int) int {
	return dist * (dist + 1) / 2
}

func day7() {
	//var inputText = "16,1,2,0,4,2,7,1,2,14"
	var inputText = downloadHelper(2021, 7)
	var numbers []int = readCommaNumbersAsSlice(inputText)
	var cost []int = make([]int, len(numbers))

	var leastCostPosition = 0
	for i := 0; i < len(numbers); i++ {
		for j := 0; j < len(numbers); j++ {
			cost[i] += Abs(numbers[i] - numbers[j])
		}
		if cost[i] < cost[leastCostPosition] {
			leastCostPosition = i
		}
	}
	// fmt.Printf("cost: %v\n", cost)
	fmt.Printf("cheapest position: %d with constant cost %d\n", leastCostPosition, cost[leastCostPosition])

	cost = make([]int, len(numbers))
	leastCostPosition = 0
	for i := 0; i < len(numbers); i++ {
		for j := 0; j < len(numbers); j++ {
			cost[i] += crabFuelUsage(Abs(numbers[i] - numbers[j]))
		}
		if cost[i] < cost[leastCostPosition] {
			leastCostPosition = i
		}
	}
	fmt.Printf("cheapest position: %d with expensive cost %d\n", leastCostPosition, cost[leastCostPosition])
}
