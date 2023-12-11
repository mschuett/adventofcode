import kotlin.math.abs

data class CosmicMap2D(var lines: List<String>, val xMax: Int, val yMax: Int) {
    companion object Factory {
        fun fromString(inputText: String): CosmicMap2D {
            val lines = inputText.trim().split('\n')
            return CosmicMap2D(lines, lines[0].indices.last, lines.indices.last)
        }
    }
    fun prettyPrint() {
        var outstr: String = "CosmicMap2D(x=${xMax}, y=${yMax})\n"
        (0..yMax).forEach { y ->
            (0..xMax).forEach { x ->
                outstr += lines[y][x]
            }
            outstr += '\n'
        }
        println(outstr)
    }
    fun expandMap(): CosmicMap2D {
        // first find columns to expand
        val expandXLines =
            (0..xMax).filter { x ->
                (0..yMax).all { y ->
                    lines[y][x] == '.'
                }
            }
        // then rows to expand
        val expandYLines =
            (0..yMax).filter { y ->
                (0..xMax).all { x ->
                    lines[y][x] == '.'
                }
            }
        // then rewrite the map
        val newMap = mutableListOf<String>()
        (0..yMax).forEach { y ->
            var newline = ""
            (0..xMax).forEach { x ->
                newline += lines[y][x]
                if (x in expandXLines)
                    newline += lines[y][x]
            }
            newMap.add(newline)
            if (y in expandYLines)
                newMap.add(newline)
        }
        return CosmicMap2D(newMap, newMap[0].indices.last, newMap.indices.last)
    }
    fun getStars(): Set<Coord2D> {
        val stars = mutableSetOf<Coord2D>()
        (0..yMax).forEach { y ->
            (0..xMax).forEach { x ->
                if (lines[y][x] == '#')
                    stars += Coord2D(x, y)
            }
        }
        return stars
    }
    fun pathLen(from: Coord2D, to: Coord2D): Int =
        abs(to.x - from.x) + abs(to.y - from.y)
}

// once again: copy&paste for part Two :-/
data class CosmicExpandedMap2D(var lines: List<String>, val xMax: Int, val yMax: Int, val expansionFactor: Int) {
    // first find columns to expand
    val expandXLines=
        (0..xMax).filter { x ->
            (0..yMax).all { y ->
                lines[y][x] == '.'
            }
        }
    // then rows to expand
    val expandYLines =
        (0..yMax).filter { y ->
            (0..xMax).all { x ->
                lines[y][x] == '.'
            }
        }

    companion object Factory {
        fun fromString(inputText: String, expansionFactor: Int): CosmicExpandedMap2D {
            val lines = inputText.trim().split('\n')
            return CosmicExpandedMap2D(lines, lines[0].indices.last, lines.indices.last, expansionFactor)
        }
    }

    private fun getStars(): Set<Coord2D> {
        val stars = mutableSetOf<Coord2D>()
        (0..yMax).forEach { y ->
            (0..xMax).forEach { x ->
                if (lines[y][x] == '#')
                    stars += Coord2D(x, y)
            }
        }
        return stars
    }

    fun getPairsOfStars(): List<Pair<Coord2D, Coord2D>> {
        val stars = getStars()
        return stars.flatMap{ a ->
          stars.map { b ->
              if (a == b)
                  null
              else if (a < b)
                  a to b
              else
                  b to a
          }
        }.filterNotNull().distinct()
    }

    fun pathLen(from: Coord2D, to: Coord2D): Int {
        val xRange = if (from.x <= to.x)
                        (from.x..to.x)
                    else
                        (to.x..from.x)
        val yRange = if (from.y <= to.y)
                        (from.y..to.y)
                    else
                        (to.y..from.y)
        val xExpansion = xRange
            .filter { it in expandXLines }
            .sumOf {
                expansionFactor - 1
            }
        val yExpansion = yRange
            .filter { it in expandYLines }
            .sumOf {
                expansionFactor - 1
            }
        val xDiff = abs(to.x - from.x)
        val yDiff = abs(to.y - from.y)
        val total = xDiff + yDiff + xExpansion + yExpansion
        return total
    }
}


private operator fun Coord2D.compareTo(b: Coord2D): Int {
    val xCmp = this.x.compareTo(b.x)
    return if (xCmp != 0)
        xCmp
    else
        this.y.compareTo(b.y)
}


fun day11(test: Boolean = true) {
    val inputText = if (test)
        """
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 11)

    // parsing
    val myMap = CosmicMap2D
        .fromString(inputText)
        .expandMap()
        .also { it.prettyPrint() }
    val stars = myMap.getStars()
    println("${stars.count()} stars: $stars")

    // Part One
    val pairs = stars.flatMap{ a ->
      stars.map { b ->
          if (a == b)
              null
          else if (a < b)
              a to b
          else
              b to a
      }
    }.filterNotNull().distinct()
    // println("${pairs.count()} pairs:")

    pairs.map {
        Triple(it.first, it.second, myMap.pathLen(it.first, it.second))
//    }.onEach {
//        println(it)
    }.sumOf {
        it.third
    }.also(::println)


    // Part Two
    val largeMap = CosmicExpandedMap2D
        .fromString(inputText, 1000000)
    println("expandXlines: ${largeMap.expandXLines}")
    println("expandYlines: ${largeMap.expandYLines}")
    largeMap.getPairsOfStars()
        .map {
            Triple(it.first, it.second, largeMap.pathLen(it.first, it.second))
//        }.onEach {
//            println(it)
        }.sumOf {
            it.third.toLong()
        }.also(::println)
}
