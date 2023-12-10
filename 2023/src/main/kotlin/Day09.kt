fun day09(test: Boolean = true) {
    val inputText = if (test)
        """
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 9)

    // parsing
    val readings = inputText.trim()
        .split("\n")
        .map {
            it.split(' ')
                .map { num -> num.toInt() }
        }

    // Part One
    readings
        .map {
            fun extrapolate(timeSeries: List<Int>): Int {
                if (timeSeries.all { num -> num == 0 }) {
                    return 0
                } else {
                    val derivation = timeSeries.zipWithNext().map { pair -> pair.second - pair.first }
                    return timeSeries.last() + extrapolate(derivation)
                }
            }
            extrapolate(it)
        }.sum().also(::println)

    // Part Two
    // did not take the time to generalize the function further
    readings
        .map {
            fun extrapolateBackwards(timeSeries: List<Int>): Int {
                if (timeSeries.all { num -> num == 0 }) {
                    return 0
                } else {
                    val derivation = timeSeries.zipWithNext().map { pair -> pair.second - pair.first }
                    return timeSeries.first() - extrapolateBackwards(derivation)
                }
            }
            extrapolateBackwards(it)
        }.sum().also(::println)
}
