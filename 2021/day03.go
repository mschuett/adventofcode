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
	/*
	   note: this is crazily overengineered. I wanted to use bit-operations,
	   even though string-operations would have been easier for every step. :(
	*/
	// var inputText = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n"
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
	for i := bitsUsed - 1; i >= 0; i-- {
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
		//fmt.Printf("bit %d; gamma  : %0.32b\n", i, gamma)
		//fmt.Printf("bit %d; epsilon: %0.32b\n", i, epsilon)
	}
	fmt.Printf("gamma: %v\n", gamma)
	fmt.Printf("epsilon: %v\n", epsilon)
	fmt.Printf("result: %v\n", gamma*epsilon)

	// Part 2
	var oxygenGeneratorRating int32 = 0
	var oxygenGeneratorRatingWorklist = make([]int32, len(reports))
	copyCount := copy(oxygenGeneratorRatingWorklist, reports)
	if copyCount != len(reports) {
		panic("unexpected copyCount")
	}
	for i := bitsUsed - 1; i >= 0; i-- {
		// check most common bit
		var countOne = 0
		for _, report := range oxygenGeneratorRatingWorklist {
			if report&(1<<i) != 0 {
				countOne++
			}
		}
		var keepWithBitOne = 2*countOne >= len(oxygenGeneratorRatingWorklist)
		// build new reduced list
		var nextIterationWorklist = make([]int32, 0)
		//fmt.Printf("bit %d, keep %v, oxygenGeneratorRatingWorklist: %v\n", i, keepWithBitOne, oxygenGeneratorRatingWorklist)
		for _, report := range oxygenGeneratorRatingWorklist {
			var isBitSet = bool(report&(1<<i) != 0)
			if (keepWithBitOne && isBitSet) || (!keepWithBitOne && !isBitSet) { // keep value
				nextIterationWorklist = append(nextIterationWorklist, report)
			}
		}
		// iteration
		oxygenGeneratorRatingWorklist = nextIterationWorklist
		if len(oxygenGeneratorRatingWorklist) == 1 {
			// one value left = found our rating
			fmt.Printf("one value left => found our oxygenGeneratorRating %v\n", oxygenGeneratorRatingWorklist)
			oxygenGeneratorRating = oxygenGeneratorRatingWorklist[0]
			break
		}
	}
	fmt.Printf("oxygenGeneratorRating: %v\n", oxygenGeneratorRating)

	var co2ScrubberRating int32 = 0
	var co2ScrubberRatingWorklist = make([]int32, len(reports))
	copy(co2ScrubberRatingWorklist, reports)
	if copyCount != len(reports) {
		panic("unexpected copyCount")
	}
	for i := bitsUsed - 1; i >= 0; i-- {
		// check most common bit
		var countOne = 0
		for _, report := range co2ScrubberRatingWorklist {
			if report&(1<<i) != 0 {
				countOne++
			}
		}
		var keepWithBitZero = 2*countOne >= len(co2ScrubberRatingWorklist)
		// build new reduced list
		var nextIterationWorklist = make([]int32, 0)
		//fmt.Printf("bit %d, keep %v, oxygenGeneratorRatingWorklist: %v\n", i, keepWithBitZero, co2ScrubberRatingWorklist)
		for _, report := range co2ScrubberRatingWorklist {
			var isBitSet = bool(report&(1<<i) != 0)
			if (keepWithBitZero && !isBitSet) || (!keepWithBitZero && isBitSet) { // keep value
				nextIterationWorklist = append(nextIterationWorklist, report)
			}
		}
		// iteration
		co2ScrubberRatingWorklist = nextIterationWorklist
		if len(co2ScrubberRatingWorklist) == 1 {
			// one value left = found our rating
			fmt.Printf("one value left => found our co2ScrubberRating %v\n", co2ScrubberRatingWorklist)
			co2ScrubberRating = co2ScrubberRatingWorklist[0]
			break
		}
	}

	fmt.Printf("co2ScrubberRating: %v\n", co2ScrubberRating)
	fmt.Printf("life support rating: %v\n", co2ScrubberRating*oxygenGeneratorRating)
}
