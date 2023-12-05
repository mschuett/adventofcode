fun day03(test: Boolean = true) {
    val inputText = if (test)
        """
        467..114..
        ...*......
        ..35...633
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...${'$'}.*....
        .664.598..
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 3)

    // parsing
    val text = inputText.trim().split('\n')

    data class Part(val symbol: Char, val x: Int, val y: Int)
    var y = 0
    var x = 0
    val numbers = ArrayList<Pair<Int,Part>>(16)
    while (y < text.size) {
        val lineLength = text[y].length-1
        while (x < lineLength) {
            while (x <= lineLength && text[y][x] == '.')
                x++
            if (x > lineLength)
                break
            if (text[y][x].isDigit()) {
                val startX = x
                while (x <= lineLength && text[y][x].isDigit())
                    x++
                val number = text[y]
                    .slice(startX until x)
                    .toInt()
                // collect all neighbouring coordinates/chars
                val neighbours = ArrayList<Part>(16)
                if (startX > 0 && text[y][startX-1] != '.')
                    neighbours.add(Part(text[y][startX-1], y, startX-1))
                if (x < text[y].length-1 && text[y][x] != '.')
                    neighbours.add(Part(text[y][x], y, x))
                if (y > 0)
                    for (x1 in (startX-1).coerceAtLeast(0) ..x.coerceAtMost(lineLength))
                        if (text[y-1][x1] != '.')
                            neighbours.add(Part(text[y-1][x1], y-1, x1))
                if (y < text.size-1)
                    for (x1 in (startX-1).coerceAtLeast(0) ..x.coerceAtMost(lineLength))
                        if (text[y+1][x1] != '.')
                            neighbours.add(Part(text[y+1][x1], y+1, x1))
                assert(neighbours.none { it.symbol.isDigit() })
                if (neighbours.isEmpty()) {
                    // println("  $number")
                }
                else {
                    assert(neighbours.size == 1)
                    numbers.add(number to neighbours.first())
                    // println("${neighbours.first().symbol} $number")
                }
            } else {
                x++
            }
        }
        x = 0
        y++
    }

    // Part One
    println(numbers.sumOf { it.first })

    // Part Two
    numbers.asSequence()
    .filter {
        it.second.symbol == '*'
//    }.also {
//        println(it)
    }.groupBy(
        keySelector = { it.second },
        valueTransform = { it.first }
    ).filter {
        it.value.size == 2
//    }.also {
//        println(it)
    }.map {
        (it.value[0] * it.value[1]).toBigInteger()
    }.sumOf {
        it
    }.also { println(it) }
}
