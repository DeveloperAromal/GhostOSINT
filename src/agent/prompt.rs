

pub fn query_builder_prompt(ch_sketch: &str) -> String {

    format!(
            r#"You are an OSINT query builder inside GhostOSINT, a reconnaissance framework.

            Your task is to generate 5 highly targeted web search queries to find information about the specific target described below.

            ## Goal
                Find real, publicly available information about THIS specific target — not generic results about the topic area.

            ## Rules
                - Focus queries entirely on the specific target (name, username, domain, IP, etc.)
                - Use exact match quotes around names/usernames to avoid unrelated results
                - Cover different angles: social profiles, code/projects, mentions, leaked data, public records
                - Use operators like site:, inurl:, intitle: only when they improve precision
                - Avoid broad topic queries (e.g. do NOT generate "cybersecurity engineer jobs")
                - Output ONLY a JSON array of 5 strings, no explanation, no markdown, no preamble

            ## Output format
                ["query 1", "query 2", "query 3", "query 4", "query 5"]

            ## STRICT
                - DONT INCLUDE ``` json ``` type things or any other code metadata 

            ## Target
                {}
            "#,
                ch_sketch
            )

}


pub fn url_builder_prompt(source_url: &str, platform: Option<&str>, username: Option<&str>) -> String {
    format!(
            r#"You are an OSINT profile URL builder.

            INPUT:
                - source_url: {}
                - platform: {}
                - username: {}

            PLATFORM PATTERNS:
                github.com/<u> | linkedin.com/in/<u> | x.com/<u> | reddit.com/u/<u>
                instagram.com/<u> | medium.com/@<u> | dev.to/<u> | gitlab.com/<u>
                npmjs.com/~<u> | pypi.org/user/<u> | hub.docker.com/u/<u>
                keybase.io/<u> | bitbucket.org/<u> | hashnode.com/@<u>
                news.ycombinator.com/user?id=<u>

            EXTRACTION:
                github.com/<u>/<repo>         → <u> = first segment
                linkedin.com/in/<u>/...       → <u> = after /in/
                x.com/<u>/status/...          → <u> = first segment
                reddit.com/u/<u>/...          → <u> = after /u/
                medium.com/@<u>/...           → <u> = after @

            CONFIDENCE:
                1.0 → username in URL, platform certain
                0.9 → username in URL, not profile page
                0.8 → username inferred from subdomain/param
                0.7 → partial match, some ambiguity
                0.5 → platform known, username guessed
                0.3 → platform uncertain, username guessed
                0.1 → extraction failed
                0.0 → unresolvable

            OUTPUT (JSON only):
                {{
                    "source_url": "",
                    "platform": "",
                    "username": "",
                    "profile_url": "",
                    "confidence": 0.0
                }}

            RULES:
                - Never fabricate username
                - Unknown platform → match against patterns
                - Failed extraction → confidence 0.1, best guess profile_url
                - Output JSON only, no explanation"#,
                        source_url,
                        platform.unwrap_or("unknown"),
                        username.unwrap_or("unknown")
                    )
}