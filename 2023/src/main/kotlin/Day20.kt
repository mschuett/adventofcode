
enum class PulseType {
    LOW,
    HIGH
}
typealias PulseTypeFromTo = Triple<PulseType,String,String>
typealias LoHighPulses = List<PulseTypeFromTo>

abstract class PulseModule(val name: String, val connected: List<String>) {
    companion object Factory {
        fun fromString(inputLine: String): PulseModule {
            val (nameText, targetText) = inputLine.split(" -> ")
            val targets = targetText.split(", ")

            return if (nameText == "broadcaster")
                BroadcastModule(nameText, targets)
            else if (nameText == "output" || nameText == "rx")
                // kludge
                OutputModule(nameText, targets)
            else if (nameText[0] == '%')
                FlipFlopModule(nameText.drop(1), targets)
            else if (nameText[0] == '&')
                ConjunctionModule(nameText.drop(1), targets)
            else
                TODO()
        }
    }
    abstract fun lowPulse(from: String): LoHighPulses
    abstract fun highPulse(from: String): LoHighPulses
    fun pulse(data: PulseTypeFromTo): LoHighPulses {
        val (type, from, to) = data
        println("$from --${type}-> $to")

        if (type == PulseType.LOW)
            return this.lowPulse(from)
        else
            return this.highPulse(from)
    }
    // unnecessary, implemented because the description hinted towards some state calculation
    abstract fun cleanState(): Boolean
}


class BroadcastModule(name: String, connected: List<String>) : PulseModule(name, connected) {
    override fun lowPulse(from: String): LoHighPulses {
        return connected.map { Triple(PulseType.LOW, this.name, it) }
    }
    override fun highPulse(from: String): LoHighPulses {
        return connected.map { Triple(PulseType.HIGH, this.name, it) }
    }
    override fun cleanState(): Boolean = true
}

class OutputModule(name: String, connected: List<String>) : PulseModule(name, connected) {
    override fun lowPulse(from: String): LoHighPulses {
        // ignore pulse, do nothing
        return listOf<PulseTypeFromTo>()
    }
    override fun highPulse(from: String): LoHighPulses {
        return listOf<PulseTypeFromTo>()
    }
    override fun cleanState(): Boolean = true
}

class FlipFlopModule(name: String, connected: List<String>, var stateOn: Boolean = false) : PulseModule(name, connected) {
    override fun lowPulse(from: String): LoHighPulses {
        stateOn = !stateOn
        if (stateOn)
            return connected.map { Triple(PulseType.HIGH, this.name, it) }
        else
            return connected.map { Triple(PulseType.LOW, this.name, it) }
    }
    override fun highPulse(from: String): LoHighPulses {
        // ignore pulse, do nothing
        return listOf<PulseTypeFromTo>()
    }
    override fun cleanState(): Boolean = (!stateOn)
}

class ConjunctionModule(name: String, connected: List<String>, var stateHigh: MutableMap<String,Boolean> = mutableMapOf()) : PulseModule(name, connected) {
    fun addInputConnection(name: String) {
        stateHigh[name] = false
    }
    override fun lowPulse(from: String): LoHighPulses {
        stateHigh[from] = false
        return connected.map { Triple(PulseType.HIGH, this.name, it) }
    }
    override fun highPulse(from: String): LoHighPulses {
        stateHigh[from] = true
        if (stateHigh.values.all { it })
            return connected.map { Triple(PulseType.LOW, this.name, it) }
        else
            return connected.map { Triple(PulseType.HIGH, this.name, it) }
    }
    override fun cleanState(): Boolean = stateHigh.values.all {!it}
}

fun day20(test: Boolean = false) {
    val inputText = if (test)
        """
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
        output -> output
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 20) + "rx -> rx"

    // parsing
    val modules = inputText.trim()
        .split("\n")
        .map { PulseModule.fromString(it) }
        .associateBy { it.name }
    // post-processing for conjunction
    val conjunctionModules = modules.values.filterIsInstance<ConjunctionModule>()
    conjunctionModules.forEach { targetMod ->
        modules.values
            .filter { srcMod ->
                targetMod.name in srcMod.connected
            }.map { it.name }
            .forEach { srcMod ->
                targetMod.addInputConnection(srcMod)
            }
    }
    // println(modules)

    // Part One
    val maxCycles = 1000
    var buttonPresses = 0
    var lowPulseCount = 0
    var highPulseCount = 0

    fun pressButton(modules: Map<String, PulseModule>): Pair<Int, Int> {
        var lowPulseCount = 0
        var highPulseCount = 0
        var pulses = mutableListOf<PulseTypeFromTo>(Triple(PulseType.LOW, "button", "broadcaster"))
        lowPulseCount += 1
        do {
            val newPulses = mutableListOf<PulseTypeFromTo>()
            for (pulse in pulses) {
                val target = modules[pulse.third]!!
                val result = target.pulse(pulse)
                newPulses += result
            }
            pulses = newPulses
            lowPulseCount  += pulses.count { it.first == PulseType.LOW }
            highPulseCount += pulses.count { it.first == PulseType.HIGH }
        } while (pulses.isNotEmpty())
        return lowPulseCount to highPulseCount
    }

    do {
        val (newLow, newHigh) = pressButton(modules)
        lowPulseCount += newLow
        highPulseCount += newHigh
        buttonPresses += 1
    } while (buttonPresses < maxCycles)

    println("after $buttonPresses button cycles")
    println("sent $lowPulseCount + $highPulseCount = ${lowPulseCount+highPulseCount} pulses => ${lowPulseCount.toLong() * highPulseCount.toLong()}")

    // Part Two
    // reset
    modules.values.filterIsInstance<ConjunctionModule>().forEach {
        it.stateHigh.keys.forEach { key ->
            it.stateHigh[key] = false
        }
    }
    modules.values.filterIsInstance<FlipFlopModule>().forEach {
        it.stateOn = false
    }
    buttonPresses = 0
    lowPulseCount = 0
    highPulseCount = 0

    val preModule = modules.values.first {
        "rx" in it.connected && it.name != "rx"
    }.name
    println("rx depends upon $preModule")
    val preModules = modules.values.filter {
        preModule in it.connected
    }.map { it.name }
    println("$preModule depends upon $preModules")

    val preModObserve = preModules.associateWith { 0 }.toMutableMap()
    do {
        val (newLow, newHigh) = pressButton(modules)
        lowPulseCount += newLow
        highPulseCount += newHigh
        buttonPresses += 1
        preModObserve.keys.forEach { name ->
            val mod = modules[name]
            if (mod is ConjunctionModule) {
                if (preModObserve[name] == 0 && mod.stateHigh.values.all { it }) {
                    println("${mod.name} switches in sub-cycle $buttonPresses")
                    preModObserve[name] = buttonPresses
                }
            }
        }
    } while (preModObserve.values.any {it == 0})
    println("found sub-cycles: $preModObserve")
}
