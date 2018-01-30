habitatConfig({

    // Enable Builder-specific features
    enable_builder: true,

    // The URL for community information
    community_url: "https://www.habitat.sh/community",

    // The URL for the demo app
    demo_app_url: "https://www.habitat.sh/demo",

    // The URL for documentation
    docs_url: "https://www.habitat.sh/docs",

    // The environment in which we're running. If "production", enable production mode
    environment: "production",

    // The URL for events
    events_url: "https://events.chef.io/events/categories/habitat/",

    // The URL for feature requests
    feature_requests_url: "https://portal.prodpad.com/24539",

    // The URL for forums
    forums_url: "https://forums.habitat.sh/",

    // The URL for the Habitat API service (including the API version.) If
    // running the API services locally with `start-builder` from the root
    // of the builder repo, this will be localhost (if running Docker for Mac or
    // Linux) or the result of `$(docker-machine ip default)` if using Docker
    // in a virtual Machine.
    habitat_api_url: "http://localhost:9636",

    // The API URL for GitHub
    github_api_url: "https://api.github.com",

    // The Habitat Builder GitHub app ID
    github_app_id: "5629",

    // The Habitat Builder GitHub app
    github_app_url: "https://github.com/apps/habitat-builder-dev",

    // GitHub Client ID for GitHubApp
    github_client_id: "Iv1.732260b62f84db15",

    // The GitHub redirect URI. Must exactly match the value of the User Authorization Callback
    // URL in the GitHub app.
    github_redirect_uri: "http://localhost:3000/",

    // The Web URL for GitHub
    github_web_url: "https://github.com",

    // The URL for the Learn Habitat
    learn_url: "https://www.habitat.sh/learn",
    // The URL for Slack

    slack_url: "http://slack.habitat.sh/",

    // The URL for the Habitat source code
    source_code_url: "https://github.com/habitat-sh/habitat",

    // The URL for status
    status_url: "https://status.habitat.sh/",

    // The URL for tutorials
    tutorials_url: "https://www.habitat.sh/tutorials",

    // The version of the software we're running. In production, this should
    // be automatically populated by Habitat
    version: "",

    // The URL for YouTube
    youtube_url: "https://www.youtube.com/playlist?list=PL11cZfNdwNyOxlvI1Kq6ae8eVBl5S3IKk",

    // The main website URL
    www_url: "https://www.habitat.sh"
});
