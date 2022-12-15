data class Sensor(val pos: Position2d, val nextBeacon: Position2d) {
    var scanDist = 0
    init {
        scanDist = pos mDist nextBeacon
    }
    override fun toString() : String = "Sensor($pos, range=$scanDist, next=$nextBeacon)"
    infix fun seesPos(other: Position2d) : Boolean = (pos mDist other) <= scanDist

}
data class SensorCave(val sensors: Iterable<Sensor>) {
    private var sensorPositions = HashSet<Position2d>()
    private var beaconPositions = HashSet<Position2d>()
    init {
        sensorPositions = sensors.map { it.pos }.toHashSet()
        beaconPositions = sensors.map { it.nextBeacon }.toHashSet()
    }
    private fun hasSensorAt(pos: Position2d) : Boolean = pos in sensorPositions
    private fun hasKnownBeaconAt(pos: Position2d) : Boolean = pos in beaconPositions

    fun getBoundingBox(outer: Boolean = true) : BoundingBox2d {
        val bb = sensors
            .flatMap { listOf(it.pos, it.nextBeacon) }
            .fold(BoundingBox2d.fromPosition2d(sensors.first().pos)) { acc, pos -> acc + pos }
        // bb has all sensors and beacons
        if (outer) {
            // now add the scan range around the scanners
            val maxDist = sensors.maxOf { it.scanDist }
            bb.minX -= maxDist
            bb.minY -= maxDist
            bb.maxX += maxDist
            bb.maxY += maxDist
        }
        return bb
    }
    fun prettyPrint(givenBB : BoundingBox2d? = null) : String = buildString {
        val bb = givenBB ?: getBoundingBox(true)
        for (y in bb.minY..bb.maxY) {
            this.append("\n%3d ".format(y))
            for (x in bb.minX..bb.maxX) {
                val curPos = Position2d(x,y)
                when {
                    hasSensorAt(curPos) -> this.append('S')
                    hasKnownBeaconAt(curPos) -> this.append('B')
                    sensors.any { it seesPos curPos } -> this.append('#')
                    else -> this.append('.')
                }
            }
        }
    }

    fun getCoverageInRow(y : Int) : Int {
        val bb = getBoundingBox()
        var covered = 0
        for (x in bb.minX..bb.maxX) {
            val curPos = Position2d(x,y)
            when {
                hasSensorAt(curPos) -> covered++
                hasKnownBeaconAt(curPos) -> 0 // does not count as covered
                sensors.any { it seesPos curPos } -> covered++
            }
        }
        return covered
    }

    fun getCoverageMissing(givenBB : BoundingBox2d) : Set<Position2d> {
        // approach: split boundingBox into NxN grid of sub-boxes with edge length of smallest scan radius
        // then quickly dismiss all sub-boxes that are completely inside a sensors scan radius (=all four corners are inside)
        fun BoundingBox2d.splitBySize(r: Int) : Sequence<BoundingBox2d> = sequence {
            for (x in minX..maxX step r)
                for (y in minY..maxY step r)
                    yield(BoundingBox2d(x, minOf(x+r, maxX), y, minOf(y+r, maxY)))
        }
        fun BoundingBox2d.fullyScanned(sensors: Iterable<Sensor>) : Boolean {
            val corners = listOf(
                Position2d(minX, minY), Position2d(maxX, minY),
                Position2d(minX, maxY), Position2d(maxX, maxY),
            )
            return sensors.any { sensor ->
                corners.all { corner ->
                    sensor seesPos corner
                }
            }
        }

        // first level split: by scan radius
        val r = sensors.minOf { it.scanDist }
        var bbList = listOf(givenBB)
        listOf(1, 10, 100, 1000, 10000).forEach { i ->
            bbList = bbList.flatMap { bb ->
                bb.splitBySize(r / i )
                    .filterNot { it.fullyScanned(sensors) }
            }
        }
        val result = mutableSetOf<Position2d>()
        for ((i, bb) in bbList.withIndex()) {
            print("\rgetCoverageMissing: now scanning $i/${bbList.size-1}")
            for (y in bb.minY..bb.maxY) {
                for (x in bb.minX..bb.maxX) {
                    val curPos = Position2d(x,y)
                    if (hasSensorAt(curPos))
                        continue
                    if (hasKnownBeaconAt(curPos))
                        continue
                    if (sensors.any { it seesPos curPos })
                        continue
                    result += curPos
                }
            }
        }
        println("... done")
        return result
    }
}

infix fun Position2d.mDist(other: Position2d) : Int = kotlin.math.abs(x - other.x) + kotlin.math.abs(y - other.y)

fun day15(test: Boolean = false) {
    val inputText = if (test)
        """
            Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            Sensor at x=9, y=16: closest beacon is at x=10, y=16
            Sensor at x=13, y=2: closest beacon is at x=15, y=3
            Sensor at x=12, y=14: closest beacon is at x=10, y=16
            Sensor at x=10, y=20: closest beacon is at x=10, y=16
            Sensor at x=14, y=17: closest beacon is at x=10, y=16
            Sensor at x=8, y=7: closest beacon is at x=2, y=10
            Sensor at x=2, y=0: closest beacon is at x=2, y=10
            Sensor at x=0, y=11: closest beacon is at x=2, y=10
            Sensor at x=20, y=14: closest beacon is at x=25, y=17
            Sensor at x=17, y=20: closest beacon is at x=21, y=22
            Sensor at x=16, y=7: closest beacon is at x=15, y=3
            Sensor at x=14, y=3: closest beacon is at x=15, y=3
            Sensor at x=20, y=1: closest beacon is at x=15, y=3
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 15)

    val sensors = inputText
        .split('\n')
        .filter { it.isNotEmpty() }
        .map {
            fun getPosInt(word: String): Int = word.split('=')[1].trim(',', ':').toInt()
            val words = it.split (' ')
            val sensorPos = Position2d(getPosInt(words[2]), getPosInt(words[3]))
            val nearestBeaconPos = Position2d(getPosInt(words[8]), getPosInt(words[9]))
            Sensor(sensorPos, nearestBeaconPos)
        }
        .also(::println)
    val cave = SensorCave(sensors)

    println(cave)
    // part A
    val searchY = if (test) 10
                  else 2_000_000
    if (test) println(cave.prettyPrint())
    println("coverage in y=$searchY is ${cave.getCoverageInRow(searchY)}")

    // part B
    val specBb = if (test) BoundingBox2d(0, 20, 0, 20)
                  else BoundingBox2d(0, 4_000_000, 0, 4_000_000)
    val caveBb = cave.getBoundingBox(false)
    val bb = specBb intersect caveBb
    println("= Bounding boxes\nspec: $specBb\ncave: $caveBb\nintersect: $bb")
    val missing = cave.getCoverageMissing(bb).toList()
    if (test) println(cave.prettyPrint(bb))
    if (missing.size == 1) {
        println("${missing.size} missing points: $missing")
        val freq : ULong = missing[0].x.toULong() * 4_000_000u + missing[0].y.toULong()
        println("tuning frequency is $freq")
    } else {
        println("ERROR, found ${missing.size} missing points")
        println(missing)
    }
}
