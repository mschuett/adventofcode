
import kotlinx.serialization.json.Json
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonPrimitive


data class SignalPacket(val number: Int = 0, val list: ArrayList<SignalPacket>? = null) : Comparable<SignalPacket> {
    override operator fun compareTo(other: SignalPacket) : Int {
        when {
            list == null && other.list == null -> {
                return number.compareTo(other.number)
            }
            list != null && other.list != null -> {
                val a = list
                val b = other.list

                for (i in 0 until minOf(a.size, b.size)) {
                    val cmp = a[i].compareTo(b[i])
                    if (cmp != 0) return cmp
                }
                // no result from element comparison
                return a.size.compareTo(b.size)
            }
            list != null && other.list == null -> {
                val cmpList = SignalPacket(list = arrayListOf(other))
                return compareTo(cmpList)
            }
            list == null && other.list != null -> {
                return -other.compareTo(this)
            }
            else -> {
                throw IllegalStateException("impossible case")
            }
        }
    }
    override fun toString() : String {
        return if (list == null) {
            number.toString()
        } else {
            "[" + list.joinToString(",") { it.toString() } + "]"
        }
    }
    companion object SignalPacketBuilder {
        fun fromString(input: String) : SignalPacket {
            // abuse JSON library for easier parsing
            val items = Json.decodeFromString<JsonArray>(input).map {
                when (it) {
                    is JsonArray -> fromString(it.toString())
                    is JsonPrimitive -> SignalPacket(it.toString().toInt())
                    else -> throw IllegalStateException("unexpected type " + it.javaClass)
                }
            } as ArrayList<SignalPacket>
            return SignalPacket(list = items)
        }
    }
}

fun day13(test: Boolean = true) {
    val inputText = if (test)
        """
            [1,1,3,1,1]
            [1,1,5,1,1]
            
            [[1],[2,3,4]]
            [[1],4]
            
            [9]
            [[8,7,6]]
            
            [[4,4],4,4]
            [[4,4],4,4,4]
            
            [7,7,7,7]
            [7,7,7]
            
            []
            [3]
            
            [[[]]]
            [[]]
            
            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 13)

    // part A
    inputText
        .split("\n\n")
        .map {
            val (a, b) = it.split('\n')
            val spA = SignalPacket.fromString(a)
            val spB = SignalPacket.fromString(b)
            spA to spB }
        .mapIndexedNotNull { index, pair ->
            val (a, b) = pair
            val cmp = a.compareTo(b)
            if (cmp < 1) {
                // println("right order in $a vs $b")
                index + 1
            } else {
                // println("wrong order in $a vs $b")
                null
            }
        }
        .also(::println)
        .sum()
        .also { println("sum of indices is $it") }

    // part B
    val allSignalPackets = inputText
        .split("\n")
        .filter { it.isNotEmpty() }
        .map { SignalPacket.fromString(it) }
        .toMutableList()
    val marker1 = SignalPacket.fromString("[[2]]")
    val marker2 = SignalPacket.fromString("[[6]]")
    allSignalPackets.add(marker1)
    allSignalPackets.add(marker2)
    allSignalPackets.sort()
    // println("sorted list:\n" + allSignalPackets.joinToString(",\n"))

    val index1 = 1 + allSignalPackets.indexOf(marker1)
    val index2 = 1 + allSignalPackets.indexOf(marker2)
    println("divider indices are $index1 and $index2 => decoder key is ${index1 * index2}")
}
