import kotlin.math.max
import kotlin.math.min

data class Coord3D(val x: Int, val y: Int, val z: Int)

data class Brick(val corners: Pair<Coord3D, Coord3D>) {
    companion object Factory {
        fun fromString(inputLine: String): Brick {
            val (from, to) = inputLine.trim()
                .split('~')
                .map { tuple -> tuple.split(',').map { it.toInt() } }
                .map { Coord3D(it[0], it[1], it[2])}
            return Brick(from to to)
        }
    }
    fun fallDown(surface: BrickSurface) {
        min(corners.first.z, corners.second.z)
        TODO("Not yet implemented")
    }
}

data class BrickSurface(val height: MutableMap<Coord2D, Int>) {
    companion object Factory {
        fun fromBricks(bricks: Collection<Brick>): BrickSurface {
            val minX = bricks.minOf { (corners) ->
                min(corners.first.x, corners.second.x)
            }
            val maxX = bricks.maxOf { (corners) ->
                max(corners.first.x, corners.second.x)
            }
            val minY = bricks.minOf { (corners) ->
                min(corners.first.y, corners.second.y)
            }
            val maxY = bricks.maxOf { (corners) ->
                max(corners.first.y, corners.second.y)
            }

            val height: MutableMap<Coord2D, Int> = (minY..maxY).flatMap { y ->
                (minX..maxX).map { x ->
                    Coord2D(x,y) to 0
                }
            }.toMap().toMutableMap()
            return BrickSurface(height)
        }
    }

}
fun day22(test: Boolean = true) {
    val inputText = if (test)
        """
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 22)

    // parsing

    // Part One
    val bricks = inputText.trim().split('\n').map {
        Brick.fromString(it)
    }.sortedBy { (corners) ->
        min(corners.first.z, corners.second.z)
    }.onEach {
        println(it)
    }
    val surface = BrickSurface.fromBricks(bricks)

    for (brick in bricks)
        brick.fallDown(surface)

    // Part Two

}
