data class BoundingBox2d(var minX: Int, var maxX: Int, var minY: Int, var maxY: Int) {
    operator fun plus(other: BoundingBox2d): BoundingBox2d =
        BoundingBox2d(
            minX = minOf(minX, other.minX),
            maxX = maxOf(maxX, other.maxX),
            minY = minOf(minY, other.minY),
            maxY = maxOf(maxY, other.maxY),
        )
    operator fun plus(pos: RegolithPosition2d): BoundingBox2d {
        return plus(fromPosition2d(pos))
    }
    operator fun plus(pos: Position2d): BoundingBox2d {
        return plus(fromPosition2d(pos))
    }
    infix fun intersect(other: BoundingBox2d): BoundingBox2d =
        BoundingBox2d(
            minX = maxOf(minX, other.minX),
            maxX = minOf(maxX, other.maxX),
            minY = maxOf(minY, other.minY),
            maxY = minOf(maxY, other.maxY),
        )

    operator fun contains(pos: RegolithPosition2d) : Boolean = pos.x in minX..maxX && pos.y in minY..maxY
    companion object BoundingBox2dBuilder {
        fun fromPosition2d(pos: RegolithPosition2d) : BoundingBox2d = BoundingBox2d(pos.x, pos.x, pos.y, pos.y)
        fun fromPosition2d(pos: Position2d) : BoundingBox2d = BoundingBox2d(pos.x, pos.x, pos.y, pos.y)
    }
}

data class RegolithPosition2d(val x: Int, val y: Int) {
    fun down(): RegolithPosition2d = RegolithPosition2d(x, y+1)
    fun downLeft(): RegolithPosition2d = RegolithPosition2d(x-1, y+1)
    fun downRight(): RegolithPosition2d = RegolithPosition2d(x+1, y+1)
    override fun toString() : String = "$x,$y"
    companion object RegolithPosition2dBuilder {
        fun fromString(input: String) : RegolithPosition2d {
            val (x, y) = input.split(',').map { it.toInt() }
            return RegolithPosition2d(x, y)
        }
    }
}

data class RegolithWall(val spec: List<RegolithPosition2d>) {
    var allPositions : Set<RegolithPosition2d> = setOf()
    init {
        // precalculate all wall positions
        require(spec.isNotEmpty())
        val set : MutableSet<RegolithPosition2d> = spec.toMutableSet()
        for (i in 1 until spec.size) {
            val a = spec[i-1]
            val b = spec[i]
            if (a.x == b.x)
                for (y in minOf(a.y, b.y)..maxOf(a.y, b.y))
                    set += RegolithPosition2d(a.x, y)
            else if (a.y == b.y)
                for (x in minOf(a.x, b.x)..maxOf(a.x, b.x))
                    set += RegolithPosition2d(x, a.y)
        }
        allPositions = set.toSet()
    }

    fun positionBoundingBox() : BoundingBox2d {
        require(spec.isNotEmpty())
        return spec.fold(BoundingBox2d.fromPosition2d(spec.first())) { acc, pos -> acc + pos }
    }
    operator fun contains(pos: RegolithPosition2d) : Boolean = allPositions.contains(pos)
}

data class RegolithCave(val start: RegolithPosition2d, val walls: List<RegolithWall>) {
    var wallPositions : MutableSet<RegolithPosition2d> = mutableSetOf()
    var sandPositions : MutableSet<RegolithPosition2d> = mutableSetOf()
    private var boundingBox = BoundingBox2d.fromPosition2d(start)

    init {
        require(walls.isNotEmpty())
        boundingBox = walls
            .map { it.positionBoundingBox() }
            .fold(BoundingBox2d.fromPosition2d(start)) { acc, pos -> acc + pos }
        wallPositions = walls
            .map { it.allPositions }
            .flatten().toMutableSet()
    }

    operator fun List<RegolithWall>.contains(pos: RegolithPosition2d) : Boolean = this.any { pos in it }
    fun addSand() : Boolean {
        var curPos = start
        if (start in sandPositions)  // for partB
            return false

        while (true) {
            val newPos = arrayListOf(curPos.down(), curPos.downLeft(), curPos.downRight())
                .firstNotNullOfOrNull { newPos ->
                    if (newPos in wallPositions || newPos in sandPositions)
                        null
                    else
                        newPos
                }
            // print("... $newPos ")
            if (newPos == null) {
                // println("stopped.")
                // sand remains at curPos
                sandPositions.add(curPos)
                return true
            }
            if (newPos !in positionBoundingBox()) {
                println("left bounding box.")
                return false
            }
            curPos = newPos
        }
    }
    fun positionBoundingBox() : BoundingBox2d {
        return boundingBox
    }
    fun prettyPrint() : String {
        val bb = positionBoundingBox()
        println("bounding box: $bb")
        var outstr = ""

        for (y in 0..bb.maxY) {
            for (x in bb.minX..bb.maxX) {
                outstr += if (start.x == x && start.y == y)
                    '+'
                else if (walls.any { RegolithPosition2d(x, y) in it })
                    '#'
                else if (RegolithPosition2d(x, y) in sandPositions)
                    'o'
                else
                    '.'
            }
            outstr += '\n'
        }
        return outstr
    }
}

fun day14(test: Boolean = true) {
    val inputText = if (test)
        """
            498,4 -> 498,6 -> 496,6
            503,4 -> 502,4 -> 502,9 -> 494,9
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 14)

    val walls = inputText
        .split('\n')
        .filter { it.isNotEmpty() }
        .map { it.split (" -> ").map { pos -> RegolithPosition2d.fromString(pos)} }
        .map { RegolithWall(it) }
        .also(::println)
    val sandOrigin = RegolithPosition2d(500, 0)
    val cave = RegolithCave(start = sandOrigin, walls = walls)

    // println(cave)
    // println(cave.prettyPrint())

    // part A
    while (cave.addSand()) {true}
    //println(cave.prettyPrint())
    println("cave has ${cave.sandPositions.size} units of sand")

    // part B
    // a little bit hacky, I simply add a very large floor wall
    val bb = cave.positionBoundingBox()
    val newWall = RegolithWall(listOf(
        RegolithPosition2d(bb.minX - 500, bb.maxY+2),
        RegolithPosition2d(bb.maxX + 500, bb.maxY+2),
        ))
    val cave2 = RegolithCave(start = sandOrigin, walls = walls + newWall)
    while (cave2.addSand()) {true}
    //println(cave2.prettyPrint())
    println("cave has ${cave2.sandPositions.size} units of sand")
}
