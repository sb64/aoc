package aoc.y2023.d1.p2

fun solve(input: String): String =
        input.lines()
                .map {
                    val digitOptions =
                            listOf(
                                    "0",
                                    "1",
                                    "2",
                                    "3",
                                    "4",
                                    "5",
                                    "6",
                                    "7",
                                    "8",
                                    "9",
                                    "one",
                                    "two",
                                    "three",
                                    "four",
                                    "five",
                                    "six",
                                    "seven",
                                    "eight",
                                    "nine"
                            )

                    val turnStrIntoDigit = { digit: String? ->
                        when (digit) {
                            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9" -> digit[0] - '0'
                            "one" -> 1
                            "two" -> 2
                            "three" -> 3
                            "four" -> 4
                            "five" -> 5
                            "six" -> 6
                            "seven" -> 7
                            "eight" -> 8
                            "nine" -> 9
                            null -> throw Exception("there is no digit in the line?")
                            else ->
                                    throw Exception(
                                            "this should be impossible, somehow we picked up an invalid \"digit\""
                                    )
                        }
                    }

                    val firstDigit = turnStrIntoDigit(it.findAnyOf(digitOptions)?.second)
                    val lastDigit = turnStrIntoDigit(it.findLastAnyOf(digitOptions)?.second)
                    firstDigit * 10 + lastDigit
                }
                .sum()
                .toString()
