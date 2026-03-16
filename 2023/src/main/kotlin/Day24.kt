import java.lang.Thread.yield
import kotlin.math.abs
import kotlin.system.exitProcess

typealias Velocity = Triple<Int, Int, Int>
data class Coord3DL(val x: Long, val y: Long, val z: Long)

data class Trajectory(val pos: Coord3DL, val vel: Velocity) {
    companion object Factory {
        fun fromString(inputLine: String): Trajectory {
            val (posPart, velPart) = inputLine.trim().replace("  ", " ").split(" @ ")
            val (xPosText, yPosText, zPosText) = posPart.split(", ").map{ it.toLong() }
            val (xVelText, yVelText, zVelText) = velPart.split(", ").map{ it.toInt() }
            return Trajectory(Coord3DL(xPosText, yPosText, zPosText), Triple(xVelText, yVelText, zVelText))
        }
    }

    private fun coordinateFormXY(): Triple<Int, Int, Long> =
        // transform from parametric equation to a,b,c for linear equation `a x + b y = c`
        Triple(-vel.second, vel.first, (pos.x * -vel.second) + (pos.y * vel.first))

    fun intersectWithXY(other: Trajectory): Pair<Long, Long> {
        val (a1, b1, c1) = this.coordinateFormXY()
        val (a2, b2, c2) = other.coordinateFormXY()
        val x = (c1 * b2 - c2 * b1).toDouble() / (a1 * b2 - a2 * b1)
        val y = (a1 * c2 - a2 * c1).toDouble() / (a1 * b2 - a2 * b1)
        return x.toLong() to y.toLong()
    }

    fun parallelWithXY(other: Trajectory): Boolean =
        (this.vel.first.toDouble() / this.vel.second) == (other.vel.first.toDouble() / other.vel.second)
}

@OptIn(ExperimentalStdlibApi::class)
fun <G> cartesianProduct(l: List<G>) = sequence<Pair<G,G>> {
    for (i in l.indices) {
        for (j in i + 1..<l.size) {
            yield(l[i] to l[j])
        }
    }
}

@OptIn(ExperimentalStdlibApi::class)
fun day24(test: Boolean = false) {
    val inputText = if (test)
        """
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 24)

    // parsing
    val stones = inputText.trim().split('\n').map { Trajectory.fromString(it) }

    // Part One
    val boundMin = if (test) 0 else 200000000000000
    val boundMax = 400000000000000
    println("bounding box from $boundMin to $boundMax")

    var intersectCount = 0
    cartesianProduct(stones)
        .forEach { (a, b) ->
            val result: Boolean
            if (a.parallelWithXY(b)) {
                result = false
                println("$a $b => parallel => $result")
            } else {
                val (xInt, yInt) = a.intersectWithXY(b)
                result = (xInt in boundMin..boundMax) && (yInt in boundMin..boundMax)
                println("$a $b => (${xInt}, ${yInt}) => $result")
            }
            if (result)
                intersectCount++
        }
    println("$intersectCount collisions in range")
    // not 27398


    // Part Two

}
