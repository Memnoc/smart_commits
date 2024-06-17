# Smart Commit

## Description

A better way to come up with good and consistent commits for you and your team.

## Why

A friend of mine once told me: "I wish there was a way something could read my code and suggest good commit messages" - well, me too!

## How

We use Github and OpenAI API to read the repository and create AI-based suggestion based on the code the programmer written for that commit.

Through the use of a git alias and a bash script (for now) we run the Rust implementation at each commit, so that we can leverage the suggestions for the specific repository we are working with.

The bash script runs the Rust binary that is being compiled with the `--release` flag in the `/target/` directory (usually not included with version control, so you won't find it here).
The script runs the binary, which contains the code to make the suggestion logic work.

**1. Create a bash script**

```
#!/bin/bash

MESSAGE=$(/home/user/projects/commit_msg_suggester/target/release/commit_msg_suggester)
if [ -z "$MESSAGE" ]
then
    echo "No commit message generated."
    exit 1
else
    git commit -m "$MESSAGE"
fi
```

**2. The script needs to be executable**

`chmod +x /home/user/script_name.sh`

**3. Configuring the alias**

This could be done globally, but I recommend doing it on the specific repo you are working in, for data sensitivity and privacy concerns (after all, AI can read your code)

`git config alias.commit-suggest '!sh /path_to_your_script/suggest_commit.sh'`

The alias allow us to run the tool with the following command:

`git commit-suggest`

To be able to run the implementation (for now) you need a script to point at the repository:

```
#!/bin/bash

# Ensure the .env file is loaded
# export $(grep -v '^#' .env | xargs)

# Print environment variables to verify they are loaded
# echo "OPENAI_API_KEY: $OPENAI_API_KEY"

# Execute the Rust binary
MESSAGE=$(~/path to your project/target/release/smart_commit)
if [ -z "$MESSAGE" ]; then
	echo "No commit message generated."
	exit 1
else
	git commit -m "$MESSAGE"
fi
```

You also need a `.env` file in the root of your project with two API keys:

`OPENAI_API_KEY={yourkey}`
`GITHUB_API_KEY={yourkey}`

The key gets loaded in the application and allow both OpenAI and Github to perform their checks.
