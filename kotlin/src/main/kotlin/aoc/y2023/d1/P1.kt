package aoc.y2023.d1.p1

fun solve(input: String): String =
        input.lines()
                .map {
                    val digits = it.filter { it.isDigit() }
                    val firstDigit = digits.first()
                    val lastDigit = digits.last()
                    (firstDigit - '0') * 10 + (lastDigit - '0')
                }
                .sum()
                .toString()
