typealias CaveRoomId = String

data class Valve(val id: CaveRoomId, val flowRate: Int, val links: Iterable<CaveRoomId>) {
    companion object ValveBuilder {
        fun fromString(input: String): Valve {
            val words = input.split (' ')
            val id = words[1]
            val rate = words[4].split('=')[1].trim(';').toInt()
            val links = words.drop(9).map {it.trim(',')}
            return Valve(id, rate, links)
        }
    }
}

data class CaveOfValves(val valves: Collection<Valve>) {
    var map: Map<CaveRoomId, Valve> = mutableMapOf()
    private var pathMap: MutableMap<Pair<CaveRoomId,CaveRoomId>,List<CaveRoomId>> = mutableMapOf()

    init {
        map = valves.associateBy { it.id }
        // pre-compute all walking paths
        for (a in valves.map {it.id}) {
            for (b in valves.map { it.id }) {
                pathMap[a to b] = map.findPath(a, b)!!
            }
        }
    }

    private fun Map<CaveRoomId, Valve>.findPath(from: CaveRoomId, to: CaveRoomId, visited: List<CaveRoomId> = arrayListOf()): List<CaveRoomId>? {
        if (from == to) {
            return listOf(to)
        }
        val viableLinks = this[from]!!.links
            .filterNot { it in visited }
        if (viableLinks.isEmpty())
            return null
        val result = viableLinks
            .mapNotNull {
                findPath(it, to, visited + from)
            }
        return if (result.isEmpty()) null
        else listOf(from) + result.minBy { it.size }
    }

    fun findOptimalActionsPartA(minutes: Int = 30, start: CaveRoomId = "AA"): Pair<Int, List<CaveRoomId>> {
        // treat all 0 valves as opened -> simplifies path finding
        return map.walkValvesAlonePartA(
            minutes,
            start,
            start,
            opened = valves.filter {it.flowRate == 0}.map {it.id},
            closed = valves.filter {it.flowRate != 0}.map {it.id},
        )
    }

    // approach: recursive search, with a state machine for every minute
    // we pick a target valve, then walk toward it, then open in
    private fun Map<CaveRoomId, Valve>.walkValvesAlonePartA(
        minLeft: Int,
        curPos: CaveRoomId,
        curTarget: CaveRoomId,
        opened: Collection<CaveRoomId>,
        closed: Collection<CaveRoomId>,
        events: List<String> = arrayListOf(),
        sumOfFLow: Int = 0
    )
            : Pair<Int, List<String>> {
        if (minLeft <= 0) {
            // println("-> $events with flow $sumOfFLow")
            return sumOfFLow to events
        }
        val cumulatedFlow = sumOfFLow + opened.sumOf { this[it]!!.flowRate }

        if (curPos == curTarget && curPos in closed)
            // reached our target
            // => open valve (and spend the minute with it)
            return walkValvesAlonePartA(minLeft - 1, curPos, curTarget, opened + curPos, closed - curPos, events + "o", cumulatedFlow)
        else if (curPos == curTarget && curPos !in closed && closed.isNotEmpty()) {
            // at target and already opened the valve (or at start pos)
            // => pick a new target
            return closed.map {newTarget ->
                val nextStep = pathMap[curPos to newTarget]!![1]
                walkValvesAlonePartA(minLeft - 1, nextStep, newTarget, opened, closed, events + nextStep, cumulatedFlow)}
                .maxBy { it.first }
        } else if (curPos == curTarget && closed.isEmpty()) {
            // no more targets available
            // => do nothing
            return walkValvesAlonePartA(minLeft - 1, curPos, curTarget, opened, closed, events + "n", cumulatedFlow)
        } else {
            // continue walking towards target
            val nextStep = pathMap[curPos to curTarget]!![1]
            return walkValvesAlonePartA(minLeft - 1, nextStep, curTarget, opened, closed, events + nextStep, cumulatedFlow)
        }
    }


    fun findOptimalActionsPartB(minutes: Int = 26, start: CaveRoomId = "AA"): Pair<Int, List<List<String>>> {
        // treat all 0 valves as opened -> simplifies path finding
        return map.walkCoopPartB(
            initialMinutes = minutes,
            minLeft = minutes,
            opened = valves.filter {it.flowRate == 0}.map {it.id}.toSet(),
            closed = valves.filter {it.flowRate != 0}.map {it.id}.toSet(),
            me = CaveActorState(start, start),
            el = CaveActorState(start, start),
        )
    }

    /*
     I mostly failed on Part B
     I kind of extended the state machine to work with two actor entities,
     thus adding lots of complexity.
     I failed to reduce the search tree enough, so this has a way too large
     search space and runs for hours on the puzzle input.
     */
    private fun Map<CaveRoomId, Valve>.walkCoopPartB(
        initialMinutes: Int,
        minLeft: Int,
        sumOfFLow: Int = 0,
        opened: Set<CaveRoomId>,
        closed: Set<CaveRoomId>,
        me: CaveActorState,
        el: CaveActorState,
    )
            : Pair<Int, List<List<String>>> {
        if (minLeft <= 0) {
//            println("-> ${me.events} flow $sumOfFLow")
//            println("   ${el.events}")
            return sumOfFLow to listOf(me.events, el.events)
        }

        val cumulatedFlow = sumOfFLow + opened.sumOf { this[it]!!.flowRate }

        // start with all simple actions that do not branch
        val newlyOpened = mutableSetOf<CaveRoomId>()
        var meDidSth = false
        var elDidSth = false

        if (me.curPos != me.curTarget) {
            meDidSth = true
            me.stepForward(pathMap)
        } else if (me.curPos in closed) {
            meDidSth = true
            me.events += " o"
            newlyOpened += me.curPos
        }

        if (el.curPos != el.curTarget) {
            elDidSth = true
            el.stepForward(pathMap)
        } else if (el.curPos in closed) {
            elDidSth = true
            el.events += " o"
            newlyOpened += el.curPos
        }

        val newClosed = closed.toMutableSet() - newlyOpened
        val newOpened = opened.toMutableSet() + newlyOpened

        // special cases: nothing to do
        if (!meDidSth && newClosed.isEmpty()) {
            meDidSth = true
            me.events += " n"
        }
        if (!elDidSth && newClosed.isEmpty()) {
            elDidSth = true
            el.events += " n"
        }

        if (meDidSth && elDidSth) {
            return walkCoopPartB(initialMinutes, minLeft - 1, cumulatedFlow, newOpened, newClosed, me, el)
        }

        // else branch into possibilities
        val possibleMes = if (meDidSth) {
            listOf(me)
        } else newClosed.map { newTarget ->
            me.copy().apply {
                curTarget = newTarget
                stepForward(pathMap)
            }
        }
        val possibleEls = if (elDidSth) {
            listOf(el)
        } else newClosed.map { newTarget ->
            el.copy().apply {
                curTarget = newTarget
                stepForward(pathMap)
            }
        }

        var prod = cartesianProduct(possibleMes, possibleEls)
        if (newClosed.size > 1)
            prod = prod.filterNot { (a, b) -> a.curTarget == b.curTarget }

        return prod
            .asSequence()
            .map { (me, el) ->
                walkCoopPartB(initialMinutes, minLeft - 1, cumulatedFlow, newOpened, newClosed, me.copy(), el.copy())
            }
            .maxBy { it.first }
    }

    data class CaveActorState(var curPos: CaveRoomId = "AA",
                              var curTarget: CaveRoomId = curPos,
                              var events: List<String> = mutableListOf()
    ) : Cloneable {
        fun stepForward(pathMap : Map<Pair<CaveRoomId,CaveRoomId>,List<CaveRoomId>>) {
            curPos = pathMap[curPos to curTarget]!![1]
            events += curPos
        }
    }
}

fun <T, U> cartesianProduct(c1: Collection<T>, c2: Collection<U>): List<Pair<T, U>> {
    return c1.flatMap { lhsElem -> c2.map { rhsElem -> lhsElem to rhsElem } }
}

fun day16(test: Boolean = true) {
    val inputText = if (test)
        """
            Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
            Valve BB has flow rate=13; tunnels lead to valves CC, AA
            Valve CC has flow rate=2; tunnels lead to valves DD, BB
            Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
            Valve EE has flow rate=3; tunnels lead to valves FF, DD
            Valve FF has flow rate=0; tunnels lead to valves EE, GG
            Valve GG has flow rate=0; tunnels lead to valves FF, HH
            Valve HH has flow rate=22; tunnel leads to valve GG
            Valve II has flow rate=0; tunnels lead to valves AA, JJ
            Valve JJ has flow rate=21; tunnel leads to valve II
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 16)

    val valves = inputText
        .split('\n')
        .filter { it.isNotEmpty() }
        .map { Valve.fromString(it) }
        .also(::println)

    val cave = CaveOfValves(valves)

    // part A
    val result = cave.findOptimalActionsPartA()
    println("$result ${result.second.size}")

    // part B
    val result2 = cave.findOptimalActionsPartB()
    println(result2.first)
    println("${result2.second.first()} ${result2.second.first().size}")
    println("${result2.second.last()} ${result2.second.last().size}")
}
