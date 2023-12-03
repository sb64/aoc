package aoc

import com.github.ajalt.clikt.core.*
import com.github.ajalt.clikt.parameters.arguments.*
import com.github.ajalt.clikt.parameters.options.*
import com.github.ajalt.clikt.parameters.types.*
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.engine.cio.*
import io.ktor.client.request.*
import kotlin.io.path.*
import kotlinx.coroutines.*

fun main(args: Array<String>) = Main().main(args)

private class Main : CliktCommand() {
    val year by argument("year", "The year to solve").choice("2023")
    val day by
            argument("day", "The day to solve")
                    .choice(*(1..25).map { it.toString() }.toTypedArray())
    val part by argument("part", "Which part to solve").choice("1", "2")
    val exampleData by
            option(
                    "-e",
                    "--example-data",
                    help = "Example data to use (if left blank, use the actual puzzle input)"
            )

    override fun run() {
        val input = exampleData ?: fetchInput(year, day)
        val output = getSolution(year, day, part, input)
        echo(output)
    }
}

private fun fetchInput(year: String, day: String): String {
    val cached_input_dir = Path("cached_input")
    if (!cached_input_dir.isDirectory()) {
        cached_input_dir.createDirectory()
    }

    val cached_file = cached_input_dir.resolve("y${year}d${day}.txt")
    if (cached_file.exists()) {
        return cached_file.readText()
    }

    var input: String = runBlocking {
        HttpClient(CIO).use { client ->
            val response =
                    client.get("https://adventofcode.com/$year/day/$day/input") {
                        cookie(name = "session", value = Path("session.txt").readText())
                    }

            response.body()
        }
    }
    input = input.trim()

    cached_file.writeText(input)

    return input
}

private fun getSolution(year: String, day: String, part: String, input: String): String =
        when (Triple(year, day, part)) {
            Triple("2023", "1", "1") -> aoc.y2023.d1.p1.solve(input)
            Triple("2023", "1", "2") -> aoc.y2023.d1.p2.solve(input)
            Triple("2023", "2", "1") -> aoc.y2023.d2.p1.solve(input)
            Triple("2023", "2", "2") -> aoc.y2023.d2.p2.solve(input)
            else -> "no solution is available"
        }
