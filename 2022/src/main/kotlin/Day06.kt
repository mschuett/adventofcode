fun findStartMarker(inputText : String, markerLength : Int) : Int {
    return inputText
        .windowed(markerLength)
        .withIndex()
        //.also { it.forEach { (idx, value) -> println("$idx: $value") } }
        .first { (_, value) -> value.toCharArray().distinct().size == markerLength }
        //.also(::println)
        .index + markerLength
}

fun day06(test: Boolean = true) {
    val inputText = if (test)
        """
           mjqjpqmgbljsphdztnvjfqwrcgsmlb
           bvwbjplbgvbhsrlpgdmjqwftvncz
           nppdvjthqldpwncqszvftbrmjlhg
           nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
           zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 6)

    // forEach is for testing, puzzle input has only one line
    inputText
        .split('\n')
        .filter { it.isNotEmpty() }
        .forEach {
            // part A
            val packetStart = findStartMarker(it, 4)
            println("start-of-packet: $packetStart")
            // part B
            val msgStart = findStartMarker(it, 14)
            println("start-of-message: $msgStart")
        }
}
