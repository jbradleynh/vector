package metadata

remap: functions: is_nullish: {
	arguments: [
		{
			name:        "value"
			description: #"The value to check for "nullishness," i.e. a useless value."#
			required:    true
			type: ["any"]
		},
	]
	internal_failure_reasons: []
	return: ["boolean"]
	category: "Check"
	description: #"""
		Determines whether the provided `value` is "nullish,"

		Nullish indicates the absence of a meaningful value. The following are considered nullish in VRL:

		* `null`
		* `"-"` (A single dash)
		* Whitespace as defined by [Unicode `White_Space` property](\(urls.unicode_whitespace))
		"""#
	examples: [
		{
			title: "Empty string is nullish"
			input: log: string: ""
			source: ".is_nullish = is_nullish(.string)"
			output: input & {log: is_nullish: true}
		},
		{
			title: "Dash is nullish"
			input: log: string: "-"
			source: ".is_nullish = is_nullish(.string)"
			output: input & {log: is_nullish: true}
		},
		{
			title: "Whitespace is nullish"
			input: log: string: "\n   \n"
			source: ".is_nullish = is_nullish(.string)"
			output: input & {log: is_nullish: true}
		},
	]
}
