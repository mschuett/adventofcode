fun day01(test: Boolean = false) {
    val inputText = if (test)
//        """
//        1abc2
//        pqr3stu8vwx
//        a1b2c3d4e5f
//        treb7uchet
//        """.trimIndent()
        """
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 1)

    // Part One
    val coords = ArrayList<Int>(30)
    for (line in inputText.split('\n')) {
        if (line.isNotEmpty()) {
            val digits = line.asIterable().filter { it.isDigit() }
            if (digits.size >= 2)
                coords.add("${digits.first()}${digits.last()}".toInt())
        }
    }
    println(coords)
    println("sum: ${coords.sum()}")

    // Part Two
    val coords2 = ArrayList<Int>(30)
    for (line in inputText.split('\n')) {
        if (line.isNotEmpty()) {
            val digits = line
                .replace("one", "on1e")
                .replace("two", "tw2o")
                .replace("three", "th3ree")
                .replace("four", "fo4ur")
                .replace("five", "fi5ve")
                .replace("six", "si6x")
                .replace("seven", "se7ven")
                .replace("eight", "ei8ght")
                .replace("nine", "ni9ne")
                .asIterable().filter { it.isDigit() }
            coords2.add("${digits.first()}${digits.last()}".toInt())
        }
    }
    println(coords2)
    println("sum: ${coords2.sum()}")
}

// 53560
