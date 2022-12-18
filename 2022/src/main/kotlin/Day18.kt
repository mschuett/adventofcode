data class BoundingBox3d(var minX: Int, var maxX: Int, var minY: Int, var maxY: Int, var minZ: Int, var maxZ: Int) {
    operator fun contains(pos: Position3d) : Boolean =
        pos.x in minX..maxX
     && pos.y in minY..maxY
     && pos.z in minZ..maxZ

    fun size() : Int = (1 + (maxX - minX)) * (1 + (maxY - minY)) * (1 + (maxZ - minZ))
}

data class Position3d(var x: Int, var y: Int, var z: Int) : Comparable<Position3d> {
    fun neighbourPositions() : List<Position3d> = listOf(
            Position3d(x-1, y  , z  ), Position3d(x+1, y  , z  ),
            Position3d(x  , y-1, z  ), Position3d(x  , y+1, z  ),
            Position3d(x  , y  , z-1), Position3d(x  , y  , z+1),
        )

    fun getConnected(cubes: Set<Position3d>, bb: BoundingBox3d): Set<Position3d> {
        val newSet : MutableSet<Position3d> = mutableSetOf(this)
        var foundNew : Boolean
        do {
            val newCubes = newSet.flatMap {
                it.neighbourPositions().filter { n ->
                    n in bb && n !in cubes && n !in newSet
                }
            }
            foundNew = newCubes.isNotEmpty()
            newSet += newCubes
        } while (foundNew)
        return newSet
    }

    override fun toString() : String = "[$x,$y,$z]"
    override operator fun compareTo(other: Position3d) : Int {
        if (x != other.x) return (x - other.x)
        if (y != other.y) return (y - other.y)
        return (z - other.z)
    }
    companion object Position3dBuilder {
        fun fromString(input: String) : Position3d {
            val (x, y, z) = input.split(',').map { it.toInt() }
            return Position3d(x, y, z)
        }
    }
}

data class LavaDroplet(val cubes: Set<Position3d>) {
    val boundingbox = BoundingBox3d(
            cubes.minOf { it.x } - 1, cubes.maxOf { it.x } + 1,
            cubes.minOf { it.y } - 1, cubes.maxOf { it.y } + 1,
            cubes.minOf { it.z } - 1, cubes.maxOf { it.z } + 1,
        )
    // all points inside the bounding box, outside of the lava droplet
    val outside = Position3d(boundingbox.minX, boundingbox.minY, boundingbox.minZ)
            .getConnected(cubes, boundingbox)

    fun countAllSurfacesPartA() : Int =
        cubes.map { cube ->
                cube.neighbourPositions().map { n ->
                    if (n in cubes) 0
                               else 1
                }.sum()
            }.sum()

    fun countOutsideSurfacesPartB() : Int {
        return cubes.map { cube ->
            cube.neighbourPositions().count { n ->
                n in outside
            }
        }.sum()
    }
}


fun day18(test: Boolean = true) {
    val inputText = if (test)
        """
            2,2,2
            1,2,2
            3,2,2
            2,1,2
            2,3,2
            2,2,1
            2,2,3
            2,2,4
            2,2,6
            1,2,5
            3,2,5
            2,1,5
            2,3,5
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 18)

    val cubePositions = inputText
        .split('\n')
        .filter { it.isNotEmpty() }
        .map {
            Position3d.fromString(it)
        }
        .toSet()
        .also(::println)

    // Part A
    LavaDroplet(cubePositions)
        .also(::println)
        .countAllSurfacesPartA()
        .also(::println)

    // Part B
    LavaDroplet(cubePositions)
        .countOutsideSurfacesPartB()
        .also(::println)
}
