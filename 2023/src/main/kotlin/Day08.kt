data class DesertPathNode(val id: String, val left: String, val right: String) {
    companion object Factory {
        fun fromString(inputText: String): DesertPathNode {
            val parts = inputText.split(" = ")
            assert(parts.size == 2)
            val paths = parts[1].removeSurrounding("(", ")").split(", ")
            assert(paths.size == 2)
            return DesertPathNode(parts[0], paths[0], paths[1])
        }
    }
}

class Desert(val path: String, nodes: List<DesertPathNode>) {
    private var map: Map<String, DesertPathNode> = nodes.associateBy { it.id }

    private fun asserts() {
        assert(map.containsKey("AAA"))
        assert(map.containsKey("ZZZ"))
        assert(path.all { it == 'L' || it == 'R' })
    }

    fun walk(start: String): Int {
        asserts()
        var stepCount = 0
        var cursor = map[start]!!
        while (!cursor.id.endsWith('Z')) {
            val dir = path[stepCount.rem(path.length)]
            when (dir) {
                'L' -> cursor = map[cursor.left]!!
                'R' -> cursor = map[cursor.right]!!
            }
            stepCount++
        }
        return stepCount
    }

    fun ghostWalk(): Long {
        // gcd & lcm from https://rosettacode.org/wiki/Least_common_multiple#Kotlin
        fun gcd(a: Long, b: Long): Long = if (b == 0L) a else gcd(b, a % b)
        fun lcm(a: Long, b: Long): Long = a / gcd(a, b) * b
        asserts()
        val startIds = map.keys.filter { it.endsWith('A') }

        return startIds.map {
            walk(it).toLong()
        }.also(::println)
        .distinct()
        .reduce {acc, i -> lcm(acc, i)}
    }
}

fun day08(test: Boolean = true) {
    val inputText = if (test)
        """
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 8)

    // Part One
    val lines = inputText.trim().split("\n\n")
    val path = lines[0].trim()
    val nodesText = lines[1].trim().split('\n')
    Desert(path, nodesText.map {DesertPathNode.fromString(it)}).walk("AAA").also(::println)

    // Part Two
    val inputText2 = if (test)
        """
        LR
        
        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
        """.trimIndent()
    else
        inputText

    val lines2 = inputText2.trim().split("\n\n")
    val path2 = lines2[0].trim()
    val nodesText2 = lines2[1].trim().split('\n')
    Desert(path2, nodesText2.map {DesertPathNode.fromString(it)}).ghostWalk().also(::println)
}
