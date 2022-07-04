// This is the BotDetector/anti-bot helper module that help to identify  and prevent bots based on a set of customizable regex patterns

use std::{collections::HashSet, fmt::Debug};
use regex::Regex;

#[derive(Debug)]
pub struct BotDetector {
    user_agents_regex: Regex,
    user_agent_patterns: HashSet<String>,

}

/// Load default bot user-agent regular expressions from a local file, unless the feature is disabled
#[cfg(feature = "include-default-BotDetector")]
const _PATTERNS: &str = include_str!("pathtoRegexPatternsFile.rgx"); // it does not exist now but we can add it if we need more patterns, another way would be reading that from our server so that we can add or remove the patterns dynamically

/// Do not load any default user-agent strings into the compiled library if feature is not enabled
#[cfg(not(feature = "include-default-BotDetector"))]
const _PATTERNS: &str = "";

impl Default for BotDetector {
    /// Constructs a new instance with default user-agent patterns.


    fn default() -> Self {
        BotDetector::new(_PATTERNS)
    }
}

impl BotDetector {
    /// Constructs a new instance with bot user-agent regular expression entries delimited by a newline
    ///
    /// All user-agent regular expressions are converted to lowercase.
    ///
    /// # Example code
    ///
    /// ```
    /// use checkbot::BotDetector;
    ///
    /// let custom_user_agent_patterns = r#"
    /// ^Googlebot-Image/
    /// bingpreview/"#;
    /// let BotDetector = BotDetector::new(custom_user_agent_patterns);
    ///
    /// assert!(BotDetector.check_bot("Googlebot-Image/1.0"));
    /// assert!(BotDetector.check_bot("Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/534+ (KHTML, like Gecko) BingPreview/1.0b"));
    /// assert!(!BotDetector.check_bot("Googlebot"));
    /// ```
    pub fn new(bot_entries: &str) -> Self {
        let user_agent_patterns = BotDetector::parse_lines(&bot_entries.to_ascii_lowercase());
        let combined_user_agent_regex = BotDetector::to_regex(&user_agent_patterns);
        BotDetector {
            user_agent_patterns,
            user_agents_regex: combined_user_agent_regex,
        }
    }

    /// Appends bot user-agent regular expressions patterns.
    ///
    /// Duplicates are ignored.
    ///
    /// # Example code
    ///
    /// ```
    ///
    /// let mut BotDetector = BotDetector::default();
    /// assert!(!BotDetector.check_bot("Mozilla/5.0 (CustomNewTestB0T /1.2)"));
    /// BotDetector.append(&[r"CustomNewTestB0T\s/\d\.\d"]);
    /// assert!(BotDetector.check_bot("Mozilla/5.0 (CustomNewTestB0T /1.2)"));
    ///
    /// let new__PATTERNS = vec!["GoogleMetaverse", "^Special/"];
    /// BotDetector.append(&new__PATTERNS);
    /// assert!(BotDetector.check_bot("Mozilla/5.0 (GoogleMetaverse/1.0)"));
    /// ```
    pub fn append(&mut self, BotDetector: &[&str]) {
        for bot in BotDetector {
            self.user_agent_patterns.insert(bot.to_ascii_lowercase());
        }
        self.update_regex()
    }


      /// Removes bot user-agent regular expressions.
    ///
    /// # Example code
    ///
    /// ```
    /// use checkbot::BotDetector;
    ///
    /// let mut BotDetector = BotDetector::default();
    ///
    ///
    /// assert!(BotDetector.check_bot("Chrome-Lighthouse"));
    /// BotDetector.remove(&["Chrome-Lighthouse"]);
    /// assert!(!BotDetector.check_bot("Chrome-Lighthouse"));
    ///
    /// let _PATTERNS_to_remove = vec!["bingpreview/", "Google Favicon"];
    /// BotDetector.remove(&_PATTERNS_to_remove);
    /// assert!(!BotDetector.check_bot("Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/534+ (KHTML, like Gecko) BingPreview/1.0b"));
    /// assert!(!BotDetector.check_bot("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/49.0.2623.75 Safari/537.36 Google Favicon"));
    /// ```
    pub fn remove(&mut self, BotDetector: &[&str]) {
        for bot in BotDetector {
            self.user_agent_patterns.remove(&bot.to_ascii_lowercase());
        }
        self.update_regex()
    }


    /// Returns `true` the user-agent is a known bot.
    ///
    /// The user-agent comparison is done using lowercase.
    ///
    /// let BotDetector = BotDetector::default();
    ///
    /// assert!(BotDetector.check_bot("Googlebot/2.1 (+http://www.google.com/bot.html)"));
    /// assert!(!BotDetector.check_bot("Dalvik/2.1.0 (Linux; U; Android 8.0.0; SM-G930F Build/R16NW)"));
    /// ```    
    pub fn check_bot(&self, user_agent: &str) -> bool {
        self.user_agents_regex
            .is_match(&user_agent.to_ascii_lowercase())
    }

    
  

    fn update_regex(&mut self) {
        self.user_agents_regex = BotDetector::to_regex(&self.user_agent_patterns)
    }

    fn parse_lines(bot_regex_entries: &str) -> HashSet<String> {
        HashSet::from_iter(
            bot_regex_entries
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(ToString::to_string),
        )
    }

    fn to_regex(regex_entries: &HashSet<String>) -> Regex {
        let pattern = regex_entries
            .iter()
            .cloned()
            .collect::<Vec<String>>()
            .join("|");

        if pattern.is_empty() {
            return Regex::new("^$").unwrap();
        }

        Regex::new(&pattern).unwrap()
    }
}

#[cfg(test)]
mod tests_BotDetector {
    use crate::BotDetector;

    static G_BotDetector: [&str; 7] = [
        "Googlebot",
        "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
        "Mozilla/5.0 (compatible; Yahoo! Slurp; http://help.yahoo.com/help/us/ysearch/slurp)",
        "Mozilla/5.0 (Linux; Android 6.0.1; Nexus 5X Build/MMB29P) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2272.96 Mobile Safari/537.36 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
        "Mozilla/5.0 (compatible; Bingbot/2.0; +http://www.bing.com/bingbot.htm)",
        "DuckDuckBot/1.0; (+http://duckduckgo.com/duckduckbot.html)",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.97 Safari/537.36 Chrome-Lighthouse"
    ];

    static N_BotDetector: [&str; 6] = [
        "",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36",
        "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 5.1; Trident/4.0; .NET CLR 1.1.4322; .NET CLR 2.0.50727; .NET CLR 3.0.4506.2152; .NET CLR 3.5.30729)",
        "Mozilla/5.0 (iPhone; CPU iPhone OS 10_3_1 like Mac OS X) AppleWebKit/603.1.30 (KHTML, like Gecko) Version/10.0 Mobile/14E304 Safari/602.1",
        "Mozilla/5.0 (Linux; Android 5.0; SAMSUNG SM-N900 Build/LRX21V) AppleWebKit/537.36 (KHTML, like Gecko) SamsungBrowser/2.1 Chrome/34.0.1847.76 Mobile Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/95.0.4638.54 Safari/537.36",
    ];

    #[test]
    fn yes_BotDetector() {
        let BotDetector = BotDetector::default();
        for bot in G_BotDetector {
            assert!(BotDetector.check_bot(bot), "Invalid bot: '{}'", bot);
        }
    }

    #[test]
    fn no_BotDetector() {
        let BotDetector = BotDetector::default();
        for bot in N_BotDetector {
            assert!(!BotDetector.check_bot(bot), "Is a bot{}", bot);
        }
    }

   

    #[test]
    fn empty_user_agent_patterns() {
        let empty_user_agent_patterns = "";
        let BotDetector = BotDetector::new(empty_user_agent_patterns);
        assert!(BotDetector.check_bot(""));
        assert!(!BotDetector.check_bot("1"));
        assert!(!BotDetector.check_bot("Googlebot"));
    }

    #[test]
    fn single_user_agent_patterns() {
        let single_user_agent_patterns = "me";
        let BotDetector = BotDetector::new(single_user_agent_patterns);
        assert!(!BotDetector.check_bot(""));
        assert!(!BotDetector.check_bot("M"));
        assert!(BotDetector.check_bot("Me"));
        assert!(!BotDetector.check_bot("Googlebot"));
    }
    #[test]
    fn add_pattern() {
        let mut BotDetector = BotDetector::default();
        assert!(!BotDetector.check_bot("Mozilla/5.0 (FancyNewTestB0T /1.2)"));
        BotDetector.append(&[r"TestCatalyzeBot\s/\d\.\d"]);
        assert!(BotDetector.check_bot("Mozilla/5.0 (FancyNewTestB0T /1.2)"));
    }

    #[test]
    fn add_multiple_patterns() {
        let mut BotDetector = BotDetector::default();
        assert!(!BotDetector.check_bot("Mozilla/5.0 (FancyNewTestB0T /1.2)"));
        assert!(!BotDetector.check_bot("Special/1.0"));
        assert!(!BotDetector.check_bot("GoogleMetaverse/2.1 (experimental)"));

        let new__PATTERNS = vec!["TestCatalyzeBot", "^GoogleMetaverse", "^Special/"];
        BotDetector.append(&new__PATTERNS);

        assert!(BotDetector.check_bot("Mozilla/5.0 (FancyNewTestB0T /1.2)"));
        assert!(BotDetector.check_bot("Special/1.0"));
        assert!(BotDetector.check_bot("GoogleMetaverse/2.1 (experimental)"));
    }

    #[test]
    fn remove_pattern() {
        let mut BotDetector = BotDetector::default();
        assert!(BotDetector.check_bot("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.97 Safari/537.36 Chrome-Lighthouse"));
        BotDetector.remove(&["Chrome-Lighthouse"]);
        assert!(!BotDetector.check_bot("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.97 Safari/537.36 Chrome-Lighthouse"));
        assert!(!BotDetector.check_bot("Chrome-Lighthouse"));
        assert!(BotDetector.check_bot("Mozilla/5.0 (Windows NT 10.0; Win64; x64) adbeat.com/policy AppleWebKit/537.36 (KHTML, like Gecko) Chrome/73.0.3683.86 Safari/537.36"));
    }

    #[test]
    fn remove_multiple_patterns() {
        let mut BotDetector = BotDetector::default();
        assert!(BotDetector.check_bot("Mozilla/5.0 (Java) outbrain"));
        assert!(BotDetector.check_bot("Mozilla/5.0 (compatible; Google-Site-Verification/1.0)"));
        assert!(BotDetector.check_bot("Datadog Agent/5.10.1"));
        assert!(BotDetector.check_bot("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.97 Safari/537.36 Chrome-Lighthouse"));


        let _PATTERNS_to_remove =
            vec!["datadog agent", "Chrome-Lighthouse", "outbrain", "google-"];
        BotDetector.remove(&_PATTERNS_to_remove);
        assert!(!BotDetector.check_bot("Mozilla/5.0 (compatible; Google-Site-Verification/1.0)"));
        assert!(!BotDetector.check_bot("Datadog Agent/5.10.1"));
        assert!(!BotDetector.check_bot("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/78.0.3904.97 Safari/537.36 Chrome-Lighthouse"));
        assert!(!BotDetector.check_bot("Mozilla/5.0 (Java) outbrain"));

    }


    #[test]
    fn custom_user_agent_patterns() {
        let custom_user_agent_patterns = "\
            ^Catalyzebot\n\
            anything\\s+bot\n\
            Numerical\\d{4}\\.\\d{4}\\.\\d{4}\\.\\d{4}";
        let BotDetector = BotDetector::new(custom_user_agent_patterns);
        assert!(BotDetector.check_bot("Anything  Bot"));
        assert!(!BotDetector.check_bot("AnythingBot"));
        assert!(BotDetector.check_bot("numerical1101.2001.3987.4781"));
        assert!(!BotDetector.check_bot("numerical1.2.3.4"));
        assert!(!BotDetector.check_bot("InvalidBot"));
        assert!(!BotDetector.check_bot("Googlebot"));
    

    }
}
