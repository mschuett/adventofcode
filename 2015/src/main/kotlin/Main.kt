
import java.math.BigInteger
import java.net.URI
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse
import java.util.Collections.max
import java.util.Collections.min
import kotlin.test.assertContains
import kotlin.test.assertEquals
import kotlin.test.assertNotNull

/* helper function to download input texts */
class InputTextDownloader {
    private val sessionId = System.getenv("ADVENT_AUTH_SESSION_ID")
    private val client = HttpClient.newBuilder()
        .followRedirects(HttpClient.Redirect.ALWAYS)
        .build()

    fun getText(day: Int) : String {
        val request = HttpRequest.newBuilder()
            .uri(URI.create("https://adventofcode.com/2015/day/${day}/input"))
            .header("Cookie", "session=$sessionId")
            .build()
        val response = client.send(request, HttpResponse.BodyHandlers.ofString())
        return response.body()
    }
}

fun day1() {
    //val inputText = "))((((("
    val inputText = InputTextDownloader().getText(1)

    var floor = 0
    var firstStepInBasement = 0
    for ((i, char) in inputText.withIndex()) {
        when (char) {
            '(' -> floor += 1
            ')' -> floor -= 1
            else -> {}
        }
        if (firstStepInBasement == 0 && floor < 0) {
            firstStepInBasement = i+1
        }
    }
    println("final floor: $floor")
    println("first basement step: $firstStepInBasement")
}

/* class for day 7 */
class Wire (
    private val name: String,
    private val gate: Gate,
    private val inputWire1: String? = null,
    private val inputWire2: String? = null,
    private val constValue: UShort? = null,
    ) {
    init {
        // println("new wire with: ${System.identityHashCode(cache)} $cache / ${System.identityHashCode(registry)} $registry")
        registry[name] = this
    }
    enum class Gate { LINK, VALUE, NOT, AND, OR, LSHIFT, RSHIFT }
    companion object Helper {
        private var cache = mutableMapOf<String, UShort?>()
        private var registry = mutableMapOf<String, Wire>()

        fun lookupWire(id: String): Wire? = registry[id]

        fun fromString(text: String): Wire {
            assertContains(text, " -> ")
            val (lpar, rpar) = text.split(" -> ")
            val wireName: String = rpar
            val expr = lpar.split(' ')
            return when (expr.size) {
                1 -> {
                    if (expr[0].all {it.isDigit()})
                        Wire(wireName, Gate.VALUE, constValue = expr[0].toUShort())
                    else
                        Wire(wireName, Gate.LINK, inputWire1 = expr[0])
                }
                2 -> {
                    assertEquals(expr[0], "NOT")
                    Wire(wireName, Gate.NOT, expr[1])
                }
                3 -> when (expr[1]) {
                    "AND" -> Wire(wireName, Gate.AND, expr[0], expr[2])
                    "OR" -> Wire(wireName, Gate.OR, expr[0], expr[2])
                    "LSHIFT" -> Wire(wireName, Gate.LSHIFT, expr[0], constValue = expr[2].toUShort())
                    "RSHIFT" -> Wire(wireName, Gate.RSHIFT, expr[0], constValue = expr[2].toUShort())
                    else -> throw Exception("cannot parse text")
                }
                else -> throw Exception("cannot parse text")
            }
        }
    }
    fun eval() : UShort? {
        fun getValueOrWire(input: String) : UShort? {
            val result : UShort?
            if (cache.containsKey(input)) {
                result = cache[input]
                // println("cache hit on $input -> $result")
                return result
            }

            if (input.all { it.isDigit() }) {
                result = input.toUShort()
                cache[name] = result
            } else {
                result = registry[input]!!.eval()
                cache[name] = result
            }
            // println("cache miss on $input -> $result")
            return result
        }
        val result = when (gate) {
            Gate.VALUE -> constValue
            Gate.LINK -> registry[inputWire1]!!.eval()!!
            Gate.NOT -> registry[inputWire1]!!.eval()!!.inv()
            Gate.AND -> {
                assertNotNull(inputWire1)
                assertNotNull(inputWire2)
                getValueOrWire(inputWire1)!!.and(
                    getValueOrWire(inputWire2)!!
                )
            }
            Gate.OR -> {
                assertNotNull(inputWire1)
                assertNotNull(inputWire2)
                getValueOrWire(inputWire1)!!.or(
                    getValueOrWire(inputWire2)!!
                )
            }
            Gate.LSHIFT -> {
                assertNotNull(inputWire1)
                getValueOrWire(inputWire1)!!.toInt().shl(constValue!!.toInt()).toUShort()
            }
            Gate.RSHIFT -> {
                assertNotNull(inputWire1)
                getValueOrWire(inputWire1)!!.toInt().shr(constValue!!.toInt()).toUShort()
            }
        }
        // println("in ${name}.eval() with $gate, using cache: ${System.identityHashCode(cache)} $cache")
        cache[name] = result
        return result
    }
}

fun day7a() {
    val test = false

    val inputText = if (test)
        """
            x AND y -> d
            x OR y -> e
            x LSHIFT 2 -> f
            y RSHIFT 2 -> g
            NOT x -> h
            NOT y -> i
            123 -> x
            456 -> y
        """.trimIndent()
    else
        InputTextDownloader().getText(7)

    for (line in inputText.split('\n')) {
        if (line.isNotEmpty()) {
            println(line)
            Wire.fromString(line)
        }
    }

    if (test) {
        println("d: ${Wire.lookupWire("d")!!.eval()}")
        println("e: ${Wire.lookupWire("e")!!.eval()}")
        println("f: ${Wire.lookupWire("f")!!.eval()}")
        println("g: ${Wire.lookupWire("g")!!.eval()}")
        println("h: ${Wire.lookupWire("h")!!.eval()}")
        println("i: ${Wire.lookupWire("i")!!.eval()}")
        println("x: ${Wire.lookupWire("x")!!.eval()}")
        println("y: ${Wire.lookupWire("y")!!.eval()}")
    } else
        println("a: ${Wire.lookupWire("a")!!.eval()}")
}

fun day7b() {
    val inputText = InputTextDownloader().getText(7)

    for (line in inputText.split('\n')) {
        if (line.isNotEmpty() and ! line.endsWith(" -> b")) {
            println(line)
            Wire.fromString(line)
        }
    }
    Wire.fromString("956 -> b")

    println("a: ${Wire.lookupWire("a")!!.eval()}")
}

fun day8() {
    val inputText = InputTextDownloader().getText(8)
    val inputText1 = """
        ""
        "abc"
        "aaa\"aaa"
        "\x27"
        """.trimIndent()

    var sumInputChars = 0
    var sumOutputChars = 0
    var sumEscapedChars = 0
    for (line in inputText.split('\n')) {
        if (line.isEmpty()) continue
        var i = 0
        var count = 0
        do {
            when (line[i]) {
                '"' -> {}
                '\\' -> {
                    i++
                    when (line[i]) {
                        'x'  -> i+=2
                        else -> {}
                    }
                    count++
                }
                else -> {
                    count++
                }
            }
            i++
        } while (i < line.length)

        i = 0
        var outstr = "\""
        do {
            when (line[i]) {
                '"' -> outstr += "\\\""
                '\\' -> outstr += "\\\\"
                else -> {
                    outstr += line[i]
                }
            }
            i++
        } while (i < line.length)
        outstr += "\""

        println("${line.length} ${count} $line --> ${outstr.length} $outstr")
        sumInputChars += line.length
        sumOutputChars += count
        sumEscapedChars += outstr.length
    }
    println("in chars: $sumInputChars out chars: $sumOutputChars escaped chars: $sumEscapedChars")
    println("diff in-out: ${sumInputChars - sumOutputChars}")
    println("diff esc-in: ${sumEscapedChars - sumInputChars}")
    // in chars: 6310 out chars: 4977 diff 1333
}

// copied from the rosetta stone website
fun <T> permute(input: List<T>): List<List<T>> {
    if (input.size == 1) return listOf(input)
    val perms = mutableListOf<List<T>>()
    val toInsert = input[0]
    for (perm in permute(input.drop(1))) {
        for (i in 0..perm.size) {
            val newPerm = perm.toMutableList()
            newPerm.add(i, toInsert)
            perms.add(newPerm)
        }
    }
    return perms
}
fun day9() {
    val inputText = InputTextDownloader().getText(9)
    val inputText1 = """
        London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141
    """.trimIndent()

    val cities = mutableSetOf<String>()
    val distances = mutableMapOf<String,MutableMap<String,Int>>()

    for (line in inputText.split('\n')) {
        if (line.isEmpty()) continue
        val (from, _ , to, _, dist) = line.split(' ')
        cities.add(from)
        cities.add(to)
        if (!distances.containsKey(from)) distances[from] = mutableMapOf<String,Int>()
        if (!distances.containsKey(to)) distances[to] = mutableMapOf<String,Int>()
        distances[from]!![to] = dist.toInt()
        distances[to]!![from] = dist.toInt()
    }
    println(cities)
    println(distances)

    val perms = permute(cities.toList())
    println("\nThere are ${perms.size} permutations")
    val allPathsWithLength = mutableMapOf<Int,List<String>>()
    for (perm in perms) {
        var pathLength = 0
        for ((from, to) in perm.windowed(2))
            pathLength += distances[from]!![to]!!
        println("$perm -> $pathLength")
        allPathsWithLength[pathLength] = perm
    }
    val shortest = min(allPathsWithLength.keys)
    println("shortest path length: $shortest, ${allPathsWithLength[shortest]}")

    val longest = max(allPathsWithLength.keys)
    println("longest path length: $longest, ${allPathsWithLength[longest]}")

}

fun day10() {
    val inputText = """
        1
        11
        21
        1211
        111221
    """.trimIndent()

    fun lookAndSay(word: String) : String {
        var outstr = StringBuffer(2*word.length)
        var i = 0

        while (i < word.length) {
            val char = word[i]
            var count = 1

            while (i+1 < word.length && char == word[i+1]) {
                i++
                count++
            }
            outstr.append("${count}${char}")
            i++
        }
        return outstr.toString()
    }

    for (line in inputText.split('\n')) {
        if (line.isEmpty()) continue
        val result = lookAndSay(line)
        println("lookAndSay($line) ==> ${result.length} $result")
    }

    // final puzzle
    val realInput = "3113322113"
    var state = realInput
    for (i in 1..100) {
        state = lookAndSay(state)
        println("iteration $i: length ${state.length}")
    }
}

/* class for day 11 */
data class Password(var word: String) {
    fun isValid() : Boolean = hasStraight() && hasNoInvalidChars() && hasPairs()

    fun hasStraight() : Boolean {
        var i = 0
        do
            if (i+2 < word.length &&
                word[i+1] == word[i]+1 &&
                word[i+2] == word[i]+2
            ) {
                return true
            }
        while (++i < word.length)
        return false
    }
    fun hasNoInvalidChars() : Boolean {
        return !(word.contains('i') ||
                 word.contains('o') ||
                 word.contains('l'))
    }
    fun hasPairs() : Boolean {
        var validPairs = 0
        var i = 0
        do if (i+1 < word.length && word[i] == word[i+1]) {
            if (++validPairs >= 2)
                return true
            i++
        } while (++i < word.length)
        return false
    }

    /* increment password, only handle wraparound at 'z', result may be invalid */
    fun simple_inc() : Password {
        var i = word.length - 1
        when (word[i]) {
            'z' -> {
                var j = 0
                while (word[i-j] == 'z') j++
                word = "${ word.substring(0, word.length-j-1)}${ word[i-j]+1 }${"a".repeat(j)}"
            }
            else -> {
                word = "${word.substring(0, word.length-1)}${word[i] + 1}"
            }
        }
        return this
    }

    /* optimize increment password, skip invalid ones */
    fun optimized_inc() : Password {
        if (!hasNoInvalidChars()) {
            var i = 0
            val wList = word.toMutableList()
            while (i < wList.count()) {
                when (wList[i]) {
                    'i', 'o', 'l' -> {
                        wList[i] = wList[i]+1
                        for (j in i+1 until wList.count()) {
                            wList[j] = 'a'
                        }
                    }
                }
                i++
            }
            word = wList.joinToString("")
            return this
        }

        val i = word.length - 1
        when (word[i]) {
            'h', 'n', 'k' -> { // skip i, o, l
                word = "${word.substring(0, word.length-1)}${word[i] + 2}"
            }
            'z' -> {
                var j = 0
                while (word[i-j] == 'z') j++
                word = "${ word.substring(0, word.length-j-1)}${ word[i-j]+1 }${"a".repeat(j)}"
            }
            else -> {
                word = "${word.substring(0, word.length-1)}${word[i] + 1}"
            }
        }
        return this
    }

    fun inc() : Password {
        do {
            optimized_inc()
            // println("inc to $word")
        }
        while (!isValid())
        return this
    }
}

fun day11() {
    val inputWord = "hxbxwxba"
    val work = Password(inputWord).inc()
    println("$inputWord  -->  ${work.word}")
    work.inc()
    println(" -->  ${work.word}")
}

fun day12aAddJsonNumbers(line: String) : Int {
    /* no json parsing, only simple pattern matching to get integers */
    fun partOfNumber(c: Char) = c.isDigit() || c == '-'

    var i = 0
    val numbers = mutableListOf<Int>()
    while (i < line.length) {
        if (!partOfNumber(line[i])) {
            i++
        } else {
            var j = 0
            while (i+j < line.length && partOfNumber(line[i+j])) {
                j++
            }
            val number = line.substring(i,i+j).toInt()
            numbers.add(number)
            i += j+1
        }
    }
    println("${numbers.sum()} <-- $numbers")
    return numbers.sum()
}

fun day12a() {
    val inputText = InputTextDownloader().getText(12)
    day12aAddJsonNumbers(inputText)

    // part two
    // I am too lazy to rewrite everything with a "real" json parser :(
}

fun day13() {
    fun <T> permute(input: List<T>): List<List<T>> {
        if (input.size == 1) return listOf(input)
        if (input.size == 2) return listOf(input)
        val perms = mutableListOf<List<T>>()
        val toInsert = input[0]
        for (perm in permute(input.drop(1))) {
            for (i in 0..perm.size) {
                val newPerm = perm.toMutableList()
                newPerm.add(i, toInsert)
                perms.add(newPerm)
            }
        }
        return perms
    }
    val inputText1 = """
        Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.
    """.trimIndent()
    val inputText = InputTextDownloader().getText(13)

    val persons = mutableSetOf<String>()
    val liking = mutableMapOf<String,MutableMap<String,Int>>()

    for (line in inputText.split('\n')) {
        if (line.isEmpty()) continue
        val wordList = line.trim('.').split(' ')
        assert(wordList.count() == 10)
        val from = wordList[0]
        val dir = wordList[2]
        val happyVal = wordList[3]
        val to = wordList[10]
        persons.add(from)
        persons.add(to)
        if (!liking.containsKey(from)) liking[from] = mutableMapOf<String,Int>()
        if (!liking.containsKey(to)) liking[to] = mutableMapOf<String,Int>()
        val happy = if (dir == "gain") happyVal.toInt() else -happyVal.toInt()
        liking[from]!![to] = happy
    }

    // part 2: add yourself
    liking["You"] = mutableMapOf<String,Int>()
    for (person in persons) {
        liking["You"]!![person] = 0
        liking[person]!!["You"] = 0
    }
    persons.add("You")
    println(persons)
    println(liking)

    var maxHappiness = 0
    for (seating in permute(persons.toList())) {
        var totalHappiness = 0
        for ((i, person) in seating.withIndex()) {
            val personRight =
                if (i + 1 == seating.size) seating[0] else seating[i + 1]
            val pairHappiness =
                liking[person]!![personRight]!! + liking[personRight]!![person]!!
            totalHappiness += pairHappiness
            // println("pair happiness ($person, $personRight): $pairHappiness = ${liking[person]!![personRight]!!} + ${liking[personRight]!![person]!!}")
        }
        println("$totalHappiness  total happiness for seating $seating")
        if (totalHappiness > maxHappiness) maxHappiness = totalHappiness
    }
    println("maximum: $maxHappiness")
}

data class Reindeer(val name: String, val speed: Int, val endurance: Int, val rest: Int) {
    var points = 0

    companion object Factory {
        fun fromString(str: String) : Reindeer {
            val words = str.split(' ')
            assertEquals(words.size, 15)
            assertEquals(words[1], "can")
            assertEquals(words[2], "fly")
            assertEquals(words[7], "seconds,")
            assertEquals(words[11], "rest")
            assertEquals(words[14], "seconds.")
            return Reindeer(words[0], words[3].toInt(), words[6].toInt(), words[13].toInt())
        }
    }
    fun distanceAtSecond(sec: Int) : Int {
        // first: calculate full fly/rest cycles
        val cycleTime = endurance + rest
        val cycleDist = speed * endurance
        val fullCycles = sec.div(cycleTime)
        var dist = fullCycles * cycleDist

        // now we only have the last partial cycle
        val timeLeft = sec - (fullCycles * cycleTime)

        if (timeLeft >= endurance)
            // full time spend flying
            dist += cycleDist
        else
            // partial time flying
            dist += timeLeft * speed
        return dist
    }
}

fun day14() {
    val inputText1 = """
        Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
    """.trimIndent()
    val inputText = InputTextDownloader().getText(14)
    val deadline = 2503
    //val deadline = 10

    val reindeers = mutableListOf<Reindeer>()
    for (line in inputText.split('\n')) {
        if (line.isEmpty()) continue
        reindeers.add(Reindeer.fromString(line))
    }
    // Part 1
    for (r in reindeers) {
        println("distance ${r.distanceAtSecond(deadline)} by $r")
    }

    // Part 2
    // very inefficient abuse of part 1 solution :)
    for (sec in 1 until deadline) {
        val statsAtSecond = mutableMapOf<Int,MutableList<Reindeer>>()
        for (r in reindeers) {
            val dist = r.distanceAtSecond(sec)
            if (!statsAtSecond.containsKey(dist))
                statsAtSecond[dist] = mutableListOf<Reindeer>()
            statsAtSecond[dist]!!.add(r)
        }
        val maxDist = max(statsAtSecond.keys)
        val winnerList = statsAtSecond[maxDist]
        for (r in winnerList!!)
            r.points ++
        println("sec $sec: max dist $maxDist, points to $winnerList")
    }
    for (r in reindeers)
      println("${r.points} for ${r.name}")
}


data class Ingredient(val name: String, val capacity: Int, val durability: Int, val flavor: Int, val texture: Int, val calories: Int) {
    fun attribs() : IntArray =
        intArrayOf(capacity, durability, flavor, texture, calories)

    companion object Factory {
        fun fromString(str: String) : Ingredient {
            val (name, attribList) = str.split(": ")
            val attribs = attribList.split(',')
            assertEquals(attribs.size, 5)
            return Ingredient(
                name,
                attribs[0].trim().split(' ')[1].toInt(),
                attribs[1].trim().split(' ')[1].toInt(),
                attribs[2].trim().split(' ')[1].toInt(),
                attribs[3].trim().split(' ')[1].toInt(),
                attribs[4].trim().split(' ')[1].toInt()
            )
        }
    }
}

fun day15() {
    val inputText1 = """
        Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    """.trimIndent()
    val inputText = InputTextDownloader().getText(15)
    val totalSpoons = 100
    val calorieTarget = 500

    val ingredients = mutableListOf<Ingredient>()
    for (line in inputText.split('\n')) {
        if (line.isEmpty()) continue
        ingredients.add(Ingredient.fromString(line))
    }
    println(ingredients)

    /* generate sequence of valid spoon distributions */
    fun generateSpoonDistSequence(ingredientCount: Int, totalSpoons: Int): Sequence<IntArray> = sequence {
        var count = BigInteger("0")
        val arr = IntArray(ingredientCount)
        val maxCount = totalSpoons.toBigInteger().pow(ingredientCount)

        while (count++ < maxCount) {
            // split into `ingredientCount` buckets
            for (bucket in 0 until ingredientCount) {
                val modulo = totalSpoons.toBigInteger().pow(bucket)
                arr[bucket] = (count / modulo).mod(totalSpoons.toBigInteger()).toInt()
            }
            if (arr.sum() == totalSpoons)
                yield(arr)
        }
    }

    var maxScore = 0
    var maxScoreWithCalories = 0
    for (dist in generateSpoonDistSequence(ingredients.size, totalSpoons)) {
        val attribs = intArrayOf(0,0,0,0,0)
        for ((i, ing) in ingredients.withIndex()) {
            for ((j, attrib) in ing.attribs().withIndex()) {
                attribs[j] += dist[i] * attrib
            }
        }
        for (i in attribs.indices)
            if (attribs[i] < 0) attribs[i] = 0

        // product of first four attributes
        val score = attribs.sliceArray(0..3).reduce(Int::times)
        if (score > maxScore)
            maxScore = score
        if (attribs[4] == calorieTarget && score > maxScoreWithCalories)
            maxScoreWithCalories = score
    }
    println("maxScore: $maxScore")
    println("maxScoreWithCalories: $maxScoreWithCalories")
}


fun main() {
    day15()
}


