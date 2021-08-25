use tide::prelude::*;

use crate::csp::filter::{
    BLOCKED_URI_FILTERS, ORIGINAL_POLICY_FILTERS, REFERRER_FILTERS, SCRIPT_SAMPLE_FILTERS,
    SOURCE_FILE_FILTERS,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CspReportContent {
    /// The URI of the resource that was blocked from loading by the
    /// Content Security Policy. If the blocked URI is from a different
    /// origin than the document-uri, then the blocked URI is truncated
    /// to contain just the scheme, host, and port.
    #[serde(alias = "blocked-uri")]
    pub blocked_uri: String,

    /// Either "enforce" or "report" depending on whether the
    /// Content-Security-Policy-Report-Only header or
    /// the Content-Security-Policy header is used.
    pub disposition: Option<String>,

    /// The URI of the document in which the violation occurred.
    #[serde(alias = "document-uri")]
    pub document_uri: String,

    /// The directive whose enforcement caused the violation. Some browsers
    /// may provide different values, such as Chrome providing
    /// style-src-elem/style-src-attr, even when the actual enforced directive
    /// was style-src.
    #[serde(alias = "effective-directive")]
    pub effective_directive: Option<String>,

    /// The original policy as specified by the Content-Security-Policy
    /// HTTP header.
    #[serde(alias = "original-policy")]
    pub original_policy: String,

    /// The referrer of the document in which the violation occurred.
    pub referrer: String,

    /// The first 40 characters of the inline script, event handler, or
    /// style that caused the violation. Only applicable to script-src*
    /// and style-src* violations, when they contain the 'report-sample'
    #[serde(alias = "script-sample")]
    pub script_sample: Option<String>,

    /// The HTTP status code of the resource on which the global object
    /// was instantiated.
    #[serde(alias = "status-code")]
    pub status_code: Option<String>,

    /// The URL of the resource where the violation occurred, stripped for reporting.
    #[serde(alias = "source-file")]
    pub source_file: Option<String>,

    /// The name of the policy section that was violated.
    #[serde(alias = "violated-directive")]
    pub violated_directive: String,
}

impl CspReportContent {
    fn is_in_blocked_uri_filters(&self) -> bool {
        BLOCKED_URI_FILTERS
            .into_iter()
            .find(|&&x| x == self.blocked_uri)
            .is_some()
    }

    fn is_in_original_policy_filters(&self) -> bool {
        ORIGINAL_POLICY_FILTERS
            .into_iter()
            .find(|&&x| x == self.original_policy)
            .is_some()
    }

    fn is_in_referrer_filters(&self) -> bool {
        REFERRER_FILTERS
            .into_iter()
            .find(|&&x| x == self.referrer)
            .is_some()
    }

    fn is_in_script_sample_filters(&self) -> bool {
        SCRIPT_SAMPLE_FILTERS
            .into_iter()
            .find(|&&x| self.script_sample.is_some() && x == self.script_sample.as_ref().unwrap())
            .is_some()
    }

    fn is_in_source_file_filters(&self) -> bool {
        SOURCE_FILE_FILTERS
            .into_iter()
            .find(|&&x| self.source_file.is_some() && x == self.source_file.as_ref().unwrap())
            .is_some()
    }

    pub fn is_in_block_list(&self) -> bool {
        self.is_in_blocked_uri_filters()
            || self.is_in_original_policy_filters()
            || self.is_in_referrer_filters()
            || self.is_in_script_sample_filters()
            || self.is_in_source_file_filters()
    }
}
