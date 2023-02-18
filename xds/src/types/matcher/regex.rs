/*!
<https://github.com/cncf/xds/blob/main/xds/type/matcher/v3/regex.proto>
*/

/// A regex matcher designed for safety when used with untrusted input.
pub struct RegexMatcher {
    engine_type: EngineType,

    /// The regex match string. The string must be supported by the configured engine.
    /// [ (validate.rules).string = {min_len : 1} ];
    regex: String
}

pub enum EngineType {
    // option (validate.required) = true;

    /// Google's RE2 regex engine.
    /// [ (validate.rules).message = {required : true} ];
    GoogleRE2(GoogleRE2)
  }

/**
Google's `RE2 <https://github.com/google/re2>`_ regex engine. The regex
string must adhere to the documented `syntax
<https://github.com/google/re2/wiki/Syntax>`_. The engine is designed to
complete execution in linear time as well as limit the amount of memory
used.

Envoy supports program size checking via runtime. The runtime keys
`re2.max_program_size.error_level` and `re2.max_program_size.warn_level`
can be set to integers as the maximum program size or complexity that a
compiled regex can have before an exception is thrown or a warning is
logged, respectively. `re2.max_program_size.error_level` defaults to 100,
and `re2.max_program_size.warn_level` has no default if unset (will not
check/log a warning).

Envoy emits two stats for tracking the program size of regexes: the
histogram `re2.program_size`, which records the program size, and the
counter `re2.exceeded_warn_level`, which is incremented each time the
program size exceeds the warn level threshold.
*/
pub struct GoogleRE2 {}
