/*!
<https://github.com/envoyproxy/envoy/blob/main/api/envoy/config/core/v3/substitution_format_string.proto>
*/

type Struct = String;

use crate::config::core::{
    base::DataSource,
    extension::TypedExtensionConfig
};

/**
Configuration to use multiple :ref:`command operators <config_access_log_command_operators>` to generate a new string in either plain text or JSON format.
*/
pub struct  SubstitutionFormatString {
    format: Format,
  
    /**
    If set to true, when command operators are evaluated to null,

    - for `text_format`, the output of the empty operator is changed from `-` to an empty string, so that empty values are omitted entirely.
    - for `json_format` the keys with null values are omitted in the output structure.
    */
    omit_empty_values: bool,
  
    /**
    Specify a `content_type` field.
    If this field is not set then `text/plain` is used for `text_format` and `application/json` is used for `json_format`.

    .. validated-code-block:: yaml
      :type-name: envoy.config.core::SubstitutionFormatString

      content_type: "text/html; charset=UTF-8"

    [(validate.rules).string = {well_known_regex: HTTP_HEADER_VALUE strict: false}];
    */
    content_type: String,
        
  
    /**
    Specifies a collection of Formatter plugins that can be called from the access log configuration.
    See the formatters extensions documentation for details.
    [#extension-category: envoy.formatter]
    */
    formatters: Vec<TypedExtensionConfig>
}

pub enum Format {
    // option (validate.required) = true;

    /**
    Specify a format with command operators to form a JSON string.
    Its details is described in :ref:`format dictionary<config_access_log_format_dictionaries>`.
    Values are rendered as strings, numbers, or boolean values as appropriate.
    Nested JSON objects may be produced by some command operators (e.g. FILTER_STATE or DYNAMIC_METADATA).
    See the documentation for a specific command operator for details.

    .. validated-code-block:: yaml
      :type-name: envoy.config.core::SubstitutionFormatString

      json_format:
        status: "%RESPONSE_CODE%"
        message: "%LOCAL_REPLY_BODY%"

    The following JSON object would be created:

    ```json
    {
      "status": 500,
      "message": "My error message"
    }
    ```
    
    [(validate.rules).message = {required: true}];
    */
    JSONFormat(Struct),

    /**
    Specify a format with command operators to form a text string.
    Its details is described in :ref:`format string <config_access_log_format_strings>`.

    For example, setting `text_format` like below,

    .. validated-code-block:: yaml
      :type-name: envoy.config.core::SubstitutionFormatString

      text_format_source:
        inline_string: "%LOCAL_REPLY_BODY%:%RESPONSE_CODE%:path=%REQ(:path)%\n"

    generates plain text similar to:

    ```text
    upstream connect error:503:path=/foo
    ```
    */
    TextFormatSource(DataSource),
}
