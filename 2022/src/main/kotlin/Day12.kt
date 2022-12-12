data class HeightMap(
    var rows: ArrayList<ArrayList<Char>>
) {
    var distances : HashMap<Position2d, Int> = hashMapOf()
    private fun getHeight(pos: Position2d) : Char {
        val (x, y) = pos
        return getHeight(x, y)
    }
    private fun getHeight(x: Int, y: Int) : Char = when (rows[y][x]) {
            'S' -> 'a'
            'E' -> 'z'
            else -> rows[y][x]
        }
    private fun getNeighbours(pos: Position2d) : List<Position2d> {
        val (x, y) = pos
        return listOfNotNull(
            if (x > 0) Position2d(x - 1, y) else null,
            if (x < rows[0].size-1) Position2d(x + 1, y) else null,
            if (y > 0) Position2d(x, y - 1) else null,
            if (y < rows.size-1) Position2d(x, y + 1) else null,
        )
    }
    private fun getSteppableNeighbours(pos: Position2d) : List<Position2d> {
        val h = getHeight(pos)
        return getNeighbours(pos)
            .filter { h - getHeight(it) <= 1 }
    }

    /* kind of the main function. At first I considered a Dijkstra search,
    * but the map is so small, that a recursive sweep is fast enough. */
    fun fillDistanceMap(start: Position2d, offset: Int = 0) {
        distances[start] = offset
        val nextSteps = getSteppableNeighbours(start)
        // println("fillDistanceMap($start, $offset, h="+getHeight(start)+") -> $nextSteps")
        nextSteps.forEach { p ->
            val dist = offset + 1
            if (p in distances && distances[p] != null && distances[p]!! <= dist) {
                // nothing
            } else {
                fillDistanceMap(p, dist)
            }
        }
    }

    fun start() : Position2d = findSpecialEntry('S')
    fun end() : Position2d = findSpecialEntry('E')
    private fun findSpecialEntry(special: Char) : Position2d {
        for ((y,row) in rows.withIndex()) {
            for ((x,c) in row.withIndex()) {
                if (c == special) return Position2d(x, y)
            }
        }
        throw Exception("special entry $special not found")
    }
    private fun findElevationPoints(height: Char) : Sequence<Position2d> =
        sequence {
            for ((y,row) in rows.withIndex()) {
                for ((x,c) in row.withIndex()) {
                    if (c == height) yield(Position2d(x, y))
                }
            }
        }
    override fun toString() : String {
        var outstring = ""

        for (row in rows) {
            for (c in row) {
                outstring += "$c"
            }
            outstring += '\n'
        }
        outstring += '\n'
        return outstring
    }

    fun distanceMapToString() : String {
        var outstring = ""

        for ((y,row) in rows.withIndex()) {
            for (x in row.indices) {
                val pos = Position2d(x, y)
                outstring += if (pos in distances) {
                    "%3d,".format(distances[pos])
                } else {
                    "   ,"
                }
            }
            outstring += '\n'
        }
        outstring += '\n'
        return outstring
    }

    fun findBestHikingStartPoint() : Position2d {
        return findElevationPoints('a')
            .filter { it in distances }
            .minBy { distances[it]!! }
    }
    companion object HeightMapBuilder {
        fun fromString(inputText: String): HeightMap {
            val rows = inputText
                .split('\n')
                .filter { it.isNotEmpty() }
                .map {
                    it.map {
                      c -> c
                    }
                } as ArrayList<ArrayList<Char>>
            return HeightMap(rows)
        }
    }
}

fun day12(test: Boolean = true) {
    val inputText = if (test)
        """
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 12)

    val arr = HeightMap.fromString(inputText)

    // Part A
    // println(arr)
    arr.fillDistanceMap(arr.end(), 0)
    println(arr.distanceMapToString())
    println(arr.distances[arr.start()])

    // Part B
    val point = arr.findBestHikingStartPoint()
    val dist = arr.distances[point]
    println("best hiking start ist $point with distance $dist")
}
