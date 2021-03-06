package metadata

remap: functions: to_syslog_facility: {
	arguments: [
		{
			name:        "value"
			description: "The facility code."
			required:    true
			type: ["integer"]
		},
	]
	internal_failure_reasons: [
		"`value` is not a valid Syslog facility code",
	]
	return: ["string"]
	category:    "Coerce"
	description: """
		Coerces the provided `value`, a Syslog [facility code](\(urls.syslog_facility)), into its corresponding
		Syslog keyword. i.e. 0 into `"kern"`, 1 into `"user", etc.
		"""
	examples: [
		{
			title: "Success"
			input: {
				syslog_facility: "4"
			}
			source: ".log_facility = to_syslog_facility(.syslog_facility)"
			output: {
				log_facility: "auth"
			}
		},
	]
}
