package main

import (
	"fmt"
	"strconv"
	"strings"
)

func readReports(inputText string) []int32 {
	var reports []int32
	for _, line := range strings.Split(strings.TrimSpace(inputText), "\n") {
		var lineContent = strings.Split(strings.TrimSpace(line), " ")
		if (len(lineContent)) != 1 { // skip unexpected format
			continue
		}

		var number, _ = strconv.ParseInt(lineContent[0], 2, 32)
		reports = append(reports, int32(number))
	}
	return reports
}

func day3() {
	//var inputText = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n"
	var inputText = downloadHelper(2021, 3)
	var bitsUsed = strings.Index(inputText, "\n")
	var reports = readReports(inputText)
	var inputCount = len(reports)
	fmt.Printf("%d: %v\n", inputCount, reports)
	for _, report := range reports {
		fmt.Printf("%0.32b\n", report)
	}

	var gamma int32 = 0
	var epsilon int32 = 0
	for i := 0; i < bitsUsed; i++ {
		var countOne = 0
		for _, report := range reports {
			var bit = report & (1 << i)
			if bit != 0 {
				countOne++
			}
		}
		if 2*countOne >= inputCount { // more ones
			gamma |= 1 << i
			epsilon &= ^(1 << i)
		} else {
			gamma &= ^(1 << i)
			epsilon |= 1 << i
		}
		fmt.Printf("bit %d; gamma  : %0.32b\n", i, gamma)
		fmt.Printf("bit %d; epsilon: %0.32b\n", i, epsilon)
	}
	fmt.Printf("gamma: %v\n", gamma)
	fmt.Printf("epsilon: %v\n", epsilon)
	fmt.Printf("result: %v\n", gamma*epsilon)
}
